use crate::vision::{process_frame, VisionState};
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

pub struct WsState {
    pub tx: Mutex<Option<mpsc::Sender<String>>>,
}

#[tauri::command]
pub async fn connect_to_pi(
    ip: String,
    app: AppHandle,
    state: State<'_, WsState>,
    vision_state: State<'_, Arc<VisionState>>,
) -> Result<(), String> {
    let url = format!("ws://{}:8765", ip);

    let (ws_stream, _) = connect_async(&url).await.map_err(|e| e.to_string())?;
    let (mut write, mut read) = ws_stream.split();

    // Create a channel for sending messages out to the websocket
    let (tx, mut rx) = mpsc::channel::<String>(32);
    *state.tx.lock().await = Some(tx.clone());
    let command_tx = tx.clone(); // For autonomous commands

    // Task for sending messages
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if write.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
        let _ = write.close().await;
    });

    let vision = vision_state.inner().clone();

    // Task for reading messages
    tokio::spawn(async move {
        let mut last_inference_time = std::time::Instant::now();
        let mut current_thermal_state = 0;
        let mut current_pan = 0.0_f32;
        let mut current_tilt = 0.0_f32;
        app.emit("ws-connected", ()).unwrap();

        while let Some(Ok(msg)) = read.next().await {
            match msg {
                Message::Text(text) => {
                    if let Ok(mut json) = serde_json::from_str::<Value>(&text) {
                        if json.get("type").and_then(|t| t.as_str()) == Some("telemetry") {
                            if let Some(frame_val) = json.get("frame") {
                                if let Some(frame_str) = frame_val.as_str() {
                                    if !frame_str.is_empty() {
                                        let is_auto = *vision.autonomous_mode.lock().await;
                                        let deadman = *vision.deadman_active.lock().await;
                                        let follow = *vision.follow_mode.lock().await;
                                        let free_roam = *vision.free_roam.lock().await;

                                        if is_auto {
                                            let target = vision.target_class.lock().await.clone();
                                            let mut resolved_boxes: Option<
                                                Vec<crate::vision::BoundingBox>,
                                            > = None;
                                            let mut max_conf = 0.0;

                                            #[cfg(not(target_os = "ios"))]
                                            {
                                                let session_guard = vision.session.lock().await;
                                                if let Some(session_mutex) = session_guard.as_ref()
                                                {
                                                    let mut session = session_mutex.lock().await;
                                                    if let Ok((boxes, shape_str, max_raw_conf)) =
                                                        process_frame(
                                                            &mut session,
                                                            frame_str,
                                                            0.4,
                                                            0.45,
                                                        )
                                                    {
                                                        json.as_object_mut().unwrap().insert(
                                                            "shape".to_string(),
                                                            serde_json::Value::String(shape_str),
                                                        );
                                                        resolved_boxes = Some(boxes);
                                                        max_conf = max_raw_conf;
                                                    }
                                                } else {
                                                    let init_err = vision.init_error.lock().await;
                                                    let err_str = init_err.clone().unwrap_or_else(
                                                        || {
                                                            "AI Model failed to load (Not Found)"
                                                                .to_string()
                                                        },
                                                    );
                                                    eprintln!("AI Model not loaded or failed to initialize: {}", err_str);
                                                    json.as_object_mut().unwrap().insert(
                                                        "error".to_string(),
                                                        serde_json::Value::String(err_str),
                                                    );
                                                }
                                            }

                                            #[cfg(target_os = "ios")]
                                            {
                                                use tauri_plugin_coreml::{
                                                    CoremlExt, InferenceRequest,
                                                };

                                                // Thermal monitor: drop frames if inference is too frequent
                                                // 0 = Nominal, 1 = Fair, 2 = Serious, 3 = Critical
                                                let min_interval = match current_thermal_state {
                                                    3 => 1000, // Critical: 1 FPS
                                                    2 => 333,  // Serious: ~3 FPS
                                                    1 => 150,  // Fair: ~6 FPS
                                                    _ => 80,   // Nominal: ~12 FPS
                                                };

                                                if last_inference_time.elapsed().as_millis()
                                                    >= min_interval
                                                {
                                                    let payload = InferenceRequest {
                                                        image_base64: frame_str.to_string(),
                                                    };

                                                    match app.coreml().run_inference(payload) {
                                                        Ok(results) => {
                                                            last_inference_time =
                                                                std::time::Instant::now();
                                                            current_thermal_state =
                                                                results.thermal_state;
                                                            let mut boxes = Vec::new();
                                                            // Get image dimensions from base64 string
                                                            let mut img_w = 320.0;
                                                            let mut img_h = 240.0;
                                                            // We know Freenove camera is 320x240, skip costly decode

                                                            for r in results.boxes {
                                                                boxes.push(
                                                                    crate::vision::BoundingBox {
                                                                        x: r.x * img_w,
                                                                        y: r.y * img_h,
                                                                        width: r.width * img_w,
                                                                        height: r.height * img_h,
                                                                        confidence: r.confidence,
                                                                        class_id: 0,
                                                                        label: r.label,
                                                                    },
                                                                );
                                                                if r.confidence > max_conf {
                                                                    max_conf = r.confidence;
                                                                }
                                                            }
                                                            let state_str =
                                                                match current_thermal_state {
                                                                    0 => "Nominal",
                                                                    1 => "Fair",
                                                                    2 => "Serious",
                                                                    _ => "Critical",
                                                                };
                                                            json.as_object_mut().unwrap().insert(
                                                                "shape".to_string(),
                                                                serde_json::Value::String(format!(
                                                                    "[CoreML - {}]",
                                                                    state_str
                                                                )),
                                                            );
                                                            resolved_boxes = Some(boxes);
                                                        }
                                                        Err(e) => {
                                                            json.as_object_mut().unwrap().insert(
                                                                "error".to_string(),
                                                                serde_json::Value::String(format!(
                                                                    "CoreML error: {:?}",
                                                                    e
                                                                )),
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    // Skip inference for this frame, retain previous boxes if needed (not implemented here since UI handles fading)
                                                    json.as_object_mut().unwrap().insert(
                                                        "shape".to_string(),
                                                        serde_json::Value::String(
                                                            "[Skipped Frame]".to_string(),
                                                        ),
                                                    );
                                                }
                                            }

                                            if let Some(boxes) = resolved_boxes {
                                                let mut cmd_to_send = None;

                                                json.as_object_mut().unwrap().insert(
                                                    "max_conf".to_string(),
                                                    serde_json::Value::Number(
                                                        serde_json::Number::from_f64(
                                                            max_conf as f64,
                                                        )
                                                        .unwrap(),
                                                    ),
                                                );

                                                if let Ok(boxes_val) = serde_json::to_value(&boxes)
                                                {
                                                    json.as_object_mut()
                                                        .unwrap()
                                                        .insert("boxes".to_string(), boxes_val);
                                                }

                                                // Find the target box
                                                let mut target_box: Option<
                                                    crate::vision::BoundingBox,
                                                > = None;
                                                let mut obstacle_box: Option<
                                                    crate::vision::BoundingBox,
                                                > = None;

                                                for b in &boxes {
                                                    if b.label == target {
                                                        if target_box.is_none()
                                                            || b.confidence
                                                                > target_box
                                                                    .as_ref()
                                                                    .unwrap()
                                                                    .confidence
                                                        {
                                                            target_box = Some(b.clone());
                                                        }
                                                    } else if b.confidence > 0.5 {
                                                        // Anything else is an obstacle if it's large and central
                                                        let cx = b.x + (b.width / 2.0);
                                                        if cx > 100.0
                                                            && cx < 220.0
                                                            && b.width * b.height > 15000.0
                                                        {
                                                            obstacle_box = Some(b.clone());
                                                        }
                                                    }
                                                }

                                                let mut throttle = 0.0;
                                                let mut steering = 0.0;

                                                if !deadman {
                                                    // Snap camera back to center when hold is released
                                                    current_pan = 0.0;
                                                    current_tilt = 0.0;
                                                }

                                                if let Some(ref tb) = target_box {
                                                    // Process camera tracking
                                                    let cx = tb.x + (tb.width / 2.0);
                                                    let cy = tb.y + (tb.height / 2.0);
                                                    let x_offset = (cx - 160.0) / 160.0; // -1 to 1
                                                    let y_offset = (cy - 120.0) / 120.0; // -1 to 1

                                                    if deadman {
                                                        if x_offset.abs() > 0.1 {
                                                            current_pan += (x_offset * 0.15) as f32;
                                                            // Pan towards target
                                                        }
                                                        if y_offset.abs() > 0.1 {
                                                            current_tilt -=
                                                                (y_offset * 0.15) as f32;
                                                            // Tilt towards target
                                                        }

                                                        current_pan = current_pan.clamp(-1.0, 1.0);
                                                        current_tilt =
                                                            current_tilt.clamp(-1.0, 1.0);
                                                    }
                                                }

                                                if deadman {
                                                    if obstacle_box.is_some() {
                                                        if free_roam {
                                                            // Obstacle avoidance: reverse and turn
                                                            throttle = -0.6;
                                                            steering = 1.0;
                                                        } else {
                                                            // Stop if obstacle directly in front
                                                            throttle = 0.0;
                                                            steering = 0.0;
                                                        }
                                                    } else if free_roam {
                                                        // Drive forward
                                                        throttle = 0.6;
                                                        steering =
                                                            if follow { current_pan } else { 0.0 };
                                                    } else if follow && target_box.is_some() {
                                                        // Follow target
                                                        steering = current_pan;
                                                        throttle = 0.6;
                                                    }
                                                }

                                                let cmd = serde_json::json!({
                                                    "type": "command",
                                                    "throttle": throttle,
                                                    "steering": steering,
                                                    "pan": current_pan,
                                                    "tilt": current_tilt
                                                });
                                                cmd_to_send = Some(cmd.to_string());

                                                if let Some(cmd_str) = cmd_to_send {
                                                    let _ = command_tx.send(cmd_str).await;
                                                }

                                                // Broadcast bounding boxes to the Pi so it can forward to iOS
                                                let boxes_msg = serde_json::json!({
                                                    "type": "bounding_boxes",
                                                    "boxes": boxes
                                                });
                                                let _ =
                                                    command_tx.send(boxes_msg.to_string()).await;
                                            }
                                        } else {
                                            // Tell frontend AI is off
                                            current_pan = 0.0;
                                            current_tilt = 0.0;
                                            json.as_object_mut().unwrap().insert(
                                                "ai_off".to_string(),
                                                serde_json::Value::Bool(true),
                                            );
                                        }
                                    }
                                }
                            }
                            let _ = app.emit("telemetry", json);
                        } else {
                            let _ = app.emit("json-message", json);
                        }
                    }
                }
                Message::Binary(bin) => {
                    let _ = app.emit("video-frame", bin.to_vec());
                }
                _ => {}
            }
        }

        app.emit("ws-disconnected", ()).unwrap();
    });

    Ok(())
}

#[tauri::command]
pub async fn send_pi_command(command: String, state: State<'_, WsState>) -> Result<(), String> {
    if let Some(tx) = &*state.tx.lock().await {
        tx.send(command).await.map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Not connected".into())
    }
}

#[tauri::command]
pub async fn disconnect_from_pi(state: State<'_, WsState>) -> Result<(), String> {
    let mut tx_lock = state.tx.lock().await;
    *tx_lock = None;
    Ok(())
}

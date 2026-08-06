use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use std::sync::Arc;
use crate::vision::{process_frame, VisionState};

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
                                        
                                        if is_auto {
                                            let target = vision.target_class.lock().await.clone();
                                            let mut resolved_boxes: Option<Vec<crate::vision::BoundingBox>> = None;
                                            let mut max_conf = 0.0;
                                            
                                            #[cfg(not(target_os = "ios"))]
                                            {
                                                let session_guard = vision.session.lock().await;
                                                if let Some(session_mutex) = session_guard.as_ref() {
                                                    let mut session = session_mutex.lock().await;
                                                    if let Ok((boxes, shape_str, max_raw_conf)) = process_frame(&mut session, frame_str, 0.4, 0.45) {
                                                        json.as_object_mut().unwrap().insert("shape".to_string(), serde_json::Value::String(shape_str));
                                                        resolved_boxes = Some(boxes);
                                                        max_conf = max_raw_conf;
                                                    }
                                                } else {
                                                    let init_err = vision.init_error.lock().await;
                                                    let err_str = init_err.clone().unwrap_or_else(|| "AI Model failed to load (Not Found)".to_string());
                                                    eprintln!("AI Model not loaded or failed to initialize: {}", err_str);
                                                    json.as_object_mut().unwrap().insert("error".to_string(), serde_json::Value::String(err_str));
                                                }
                                            }
                                            
                                            #[cfg(target_os = "ios")]
                                            {
                                                use tauri_plugin_coreml::{CoremlExt, InferenceRequest};
                                                
                                                let payload = InferenceRequest {
                                                    image_base64: frame_str.to_string(),
                                                };
                                                
                                                match app.coreml().run_inference(payload) {
                                                    Ok(results) => {
                                                        let mut boxes = Vec::new();
                                                        // Get image dimensions from base64 string
                                                        let mut img_w = 640.0;
                                                        let mut img_h = 480.0;
                                                        if let Ok(img_bytes) = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, frame_str) {
                                                            if let Ok(img) = image::load_from_memory(&img_bytes) {
                                                                img_w = img.width() as f32;
                                                                img_h = img.height() as f32;
                                                            }
                                                        }
                                                        
                                                        for r in results {
                                                            boxes.push(crate::vision::BoundingBox {
                                                                x: r.x * img_w,
                                                                y: r.y * img_h,
                                                                width: r.width * img_w,
                                                                height: r.height * img_h,
                                                                confidence: r.confidence,
                                                                class_id: 0,
                                                                label: r.label,
                                                            });
                                                            if r.confidence > max_conf { max_conf = r.confidence; }
                                                        }
                                                        json.as_object_mut().unwrap().insert("shape".to_string(), serde_json::Value::String("[CoreML]".to_string()));
                                                        resolved_boxes = Some(boxes);
                                                    }
                                                    Err(e) => {
                                                        json.as_object_mut().unwrap().insert("error".to_string(), serde_json::Value::String(format!("CoreML error: {:?}", e)));
                                                    }
                                                }
                                            }
                                            
                                            if let Some(boxes) = resolved_boxes {
                                                let mut cmd_to_send = None;
                                                
                                                json.as_object_mut().unwrap().insert("max_conf".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(max_conf as f64).unwrap()));
                                                
                                                if let Ok(boxes_val) = serde_json::to_value(&boxes) {
                                                    json.as_object_mut().unwrap().insert("boxes".to_string(), boxes_val);
                                                }
                                                
                                                // Find the target box
                                                let mut target_box: Option<crate::vision::BoundingBox> = None;
                                                let mut obstacle_box: Option<crate::vision::BoundingBox> = None;
                                                
                                                for b in &boxes {
                                                    if b.label == target {
                                                        if target_box.is_none() || b.confidence > target_box.as_ref().unwrap().confidence {
                                                            target_box = Some(b.clone());
                                                        }
                                                    } else if b.confidence > 0.5 {
                                                        // Anything else is an obstacle if it's large and central
                                                        let cx = b.x + (b.width / 2.0);
                                                        if cx > 200.0 && cx < 440.0 && b.width * b.height > 60000.0 {
                                                            obstacle_box = Some(b.clone());
                                                        }
                                                    }
                                                }
                                                
                                                let mut throttle = 0.0;
                                                let mut steering = 0.0;
                                                
                                                if obstacle_box.is_some() {
                                                    // Stop if obstacle directly in front
                                                    throttle = 0.0;
                                                    steering = 0.0;
                                                } else if let Some(tb) = target_box {
                                                    // Center is x=320 (assuming 640x640 frame)
                                                    let cx = tb.x + (tb.width / 2.0);
                                                    let offset = (cx - 320.0) / 320.0; // -1 to 1
                                                    
                                                    // Deadzone for steering
                                                    if offset.abs() > 0.1 {
                                                        steering = offset;
                                                    }
                                                    
                                                    let area = tb.width * tb.height;
                                                    if area < 30000.0 {
                                                        throttle = 0.6; // move forward
                                                    } else if area > 80000.0 {
                                                        throttle = -0.6; // too close, back up
                                                    } else {
                                                        throttle = 0.0;
                                                    }
                                                }
                                                
                                                let cmd = serde_json::json!({
                                                    "type": "command",
                                                    "throttle": throttle,
                                                    "steering": steering
                                                });
                                                cmd_to_send = Some(cmd.to_string());
                                                
                                                if let Some(cmd_str) = cmd_to_send {
                                                    let _ = command_tx.send(cmd_str).await;
                                                }
                                            }
                                        } else {
                                            // Tell frontend AI is off
                                            json.as_object_mut().unwrap().insert("ai_off".to_string(), serde_json::Value::Bool(true));
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

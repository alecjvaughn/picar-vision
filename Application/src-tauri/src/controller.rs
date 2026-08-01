use crate::websocket::WsState;
use crate::ControlSettings;
use gilrs::{Axis, Button, Event, EventType, Gilrs};
use serde_json::{json, Value};
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager};

pub fn start_controller_loop(app_handle: AppHandle) {
    std::thread::spawn(move || {
        let mut gilrs = match Gilrs::new() {
            Ok(g) => g,
            Err(e) => {
                println!("Failed to initialize Gilrs: {}", e);
                return;
            }
        };

        let mut auto_active_gamepad = None;
        let mut active_gamepad = None;
        let mut pan: f32 = 0.0;
        let mut tilt: f32 = 0.0;
        
        let mut last_throttle = 0.0;
        let mut last_steering = 0.0;
        let mut last_pan = 0.0;
        let mut last_tilt = 0.0;
        
        let mut last_status_emit = Instant::now();

        loop {
            let settings = app_handle.state::<ControlSettings>();
            let force_active_gamepad = settings.force_active_gamepad.load(Ordering::Relaxed);

            // Examine new events
            while let Some(Event { id, event, .. }) = gilrs.next_event() {
                auto_active_gamepad = Some(id);

                match event {
                    EventType::Connected => {
                        let name = gilrs.gamepad(id).name().to_string();
                        // Status will be handled by the periodic loop
                    }
                    EventType::Disconnected => {
                        if Some(id) == auto_active_gamepad {
                            auto_active_gamepad = None;
                        }
                    }
                    EventType::ButtonPressed(button, _) => {
                        let button_name = format!("{:?}", button);
                        let _ = app_handle.emit(
                            "gamepad-input",
                            json!({"button": button_name, "pressed": true}),
                        );

                        if button == Button::RightTrigger2 {
                            pan = 0.0;
                            tilt = 0.0;
                        }
                    }
                    EventType::ButtonReleased(button, _) => {
                        let button_name = format!("{:?}", button);
                        let _ = app_handle.emit(
                            "gamepad-input",
                            json!({"button": button_name, "pressed": false}),
                        );
                    }
                    _ => {}
                }
            }

            // Resolve active gamepad
            if force_active_gamepad == -2 {
                active_gamepad = None;
            } else if force_active_gamepad >= 0 {
                let target_id = force_active_gamepad as usize;
                active_gamepad = None;
                for (id, _) in gilrs.gamepads() {
                    let id_usize: usize = id.into();
                    if id_usize == target_id {
                        active_gamepad = Some(id);
                        break;
                    }
                }
            } else {
                active_gamepad = auto_active_gamepad;
            }

            // Periodic status emit for gamepads list and active status
            if last_status_emit.elapsed() > Duration::from_secs(1) {
                let mut pads = Vec::new();
                for (id, gamepad) in gilrs.gamepads() {
                    let id_usize: usize = id.into();
                    let name = gamepad.name().to_string();
                    let power = gamepad.power_info();
                    let battery = match power {
                        gilrs::PowerInfo::Unknown => "Unknown".to_string(),
                        gilrs::PowerInfo::Wired => "Wired".to_string(),
                        gilrs::PowerInfo::Discharging(lvl) | gilrs::PowerInfo::Charging(lvl) => format!("{}%", lvl),
                        gilrs::PowerInfo::Charged => "100%".to_string(),
                    };
                    
                    let mut joycon_type = "Standard";
                    if name.contains("Joy-Con (L)") { joycon_type = "Single L"; }
                    else if name.contains("Joy-Con (R)") { joycon_type = "Single R"; }
                    else if name.contains("Joy-Con (L/R)") || name.contains("Joy-Con") { joycon_type = "Dual"; }
                    
                    pads.push(json!({
                        "id": id_usize,
                        "name": name,
                        "battery": battery,
                        "joyconType": joycon_type,
                        "active": Some(id) == active_gamepad
                    }));
                }
                
                let _ = app_handle.emit("gamepads-list", json!(pads));
                
                if let Some(id) = active_gamepad {
                    let id_usize: usize = id.into();
                    if let Some(p) = pads.iter().find(|p| p["id"] == json!(id_usize)) {
                        let has_l = pads.iter().any(|p| p["joyconType"] == "Single L");
                        let has_r = pads.iter().any(|p| p["joyconType"] == "Single R");
                        
                        if force_active_gamepad == -1 && has_l && has_r {
                            let _ = app_handle.emit(
                                "controller-status",
                                json!({"connected": true, "name": "Nintendo Joy-Cons (Merged)", "battery": p["battery"], "joyconType": "Dual (Merged)"}),
                            );
                        } else {
                            let _ = app_handle.emit(
                                "controller-status",
                                json!({"connected": true, "name": p["name"], "battery": p["battery"], "joyconType": p["joyconType"]}),
                            );
                        }
                    }
                } else {
                    let _ = app_handle.emit("controller-status", json!({"connected": false}));
                }
                
                last_status_emit = Instant::now();
            }

            // Determine which gamepads to process
            let mut valid_gamepads = Vec::new();
            if force_active_gamepad >= 0 {
                if let Some(id) = active_gamepad { valid_gamepads.push(id); }
            } else if force_active_gamepad == -1 {
                for (id, _) in gilrs.gamepads() { valid_gamepads.push(id); }
            }

            if !valid_gamepads.is_empty() {
                let motor_scale = settings.motor_speed.load(Ordering::Relaxed) as f32 / 100.0;
                let servo_sens = settings.servo_sensitivity.load(Ordering::Relaxed) as f32 / 100.0;
                let viewport_turn = settings.viewport_turn.load(Ordering::Relaxed);

                let lx_c = settings.lx_center.load(Ordering::Relaxed) as f32 / 1000.0;
                let ly_c = settings.ly_center.load(Ordering::Relaxed) as f32 / 1000.0;
                let rx_c = settings.rx_center.load(Ordering::Relaxed) as f32 / 1000.0;
                let ry_c = settings.ry_center.load(Ordering::Relaxed) as f32 / 1000.0;
                
                let lx_d = settings.lx_deadzone.load(Ordering::Relaxed) as f32 / 1000.0;
                let ly_d = settings.ly_deadzone.load(Ordering::Relaxed) as f32 / 1000.0;
                let rx_d = settings.rx_deadzone.load(Ordering::Relaxed) as f32 / 1000.0;
                let ry_d = settings.ry_deadzone.load(Ordering::Relaxed) as f32 / 1000.0;

                let mut merged_throttle = 0.0_f32;
                let mut merged_steering = 0.0_f32;
                let mut merged_r_pan = 0.0_f32;
                let mut merged_r_tilt = 0.0_f32;
                
                let mut merged_dpad_x = 0.0_f32;
                let mut merged_dpad_y = 0.0_f32;
                let mut btn_dpad_left = false;
                let mut btn_dpad_right = false;
                let mut btn_dpad_up = false;
                let mut btn_dpad_down = false;

                for id in valid_gamepads {
                    let gamepad = gilrs.gamepad(id);
                    let mut throttle = gamepad.value(Axis::LeftStickY) - ly_c;
                    let mut steering = gamepad.value(Axis::LeftStickX) - lx_c;
                    let mut r_pan = gamepad.value(Axis::RightStickX) - rx_c;
                    let mut r_tilt = gamepad.value(Axis::RightStickY) - ry_c;

                    if throttle.abs() < ly_d { throttle = 0.0; } else { throttle = throttle.signum() * (throttle.abs() - ly_d) / (1.0 - ly_d.abs()); }
                    if steering.abs() < lx_d { steering = 0.0; } else { steering = steering.signum() * (steering.abs() - lx_d) / (1.0 - lx_d.abs()); }
                    if r_pan.abs() < rx_d { r_pan = 0.0; } else { r_pan = r_pan.signum() * (r_pan.abs() - rx_d) / (1.0 - rx_d.abs()); }
                    if r_tilt.abs() < ry_d { r_tilt = 0.0; } else { r_tilt = r_tilt.signum() * (r_tilt.abs() - ry_d) / (1.0 - ry_d.abs()); }

                    if gamepad.name().contains("Joy-Con (R)") {
                        if throttle.abs() > 0.0 {
                            r_tilt = throttle;
                            throttle = 0.0;
                        }
                        if steering.abs() > 0.0 {
                            r_pan = steering;
                            steering = 0.0;
                        }
                    }

                    if throttle.abs() > merged_throttle.abs() { merged_throttle = throttle; }
                    if steering.abs() > merged_steering.abs() { merged_steering = steering; }
                    if r_pan.abs() > merged_r_pan.abs() { merged_r_pan = r_pan; }
                    if r_tilt.abs() > merged_r_tilt.abs() { merged_r_tilt = r_tilt; }

                    let dpad_x = gamepad.axis_data(Axis::DPadX).map(|a| a.value()).unwrap_or(0.0);
                    let dpad_y = gamepad.axis_data(Axis::DPadY).map(|a| a.value()).unwrap_or(0.0);

                    if dpad_x.abs() > merged_dpad_x.abs() { merged_dpad_x = dpad_x; }
                    if dpad_y.abs() > merged_dpad_y.abs() { merged_dpad_y = dpad_y; }

                    btn_dpad_left |= gamepad.is_pressed(Button::DPadLeft) || gamepad.is_pressed(Button::West);
                    btn_dpad_right |= gamepad.is_pressed(Button::DPadRight) || gamepad.is_pressed(Button::East);
                    btn_dpad_up |= gamepad.is_pressed(Button::DPadUp) || gamepad.is_pressed(Button::North);
                    btn_dpad_down |= gamepad.is_pressed(Button::DPadDown) || gamepad.is_pressed(Button::South);
                }

                merged_throttle *= motor_scale;
                merged_steering *= motor_scale;

                pan = 0.0;
                tilt = 0.0;
                
                if btn_dpad_left || merged_dpad_x < -0.5 { pan = -servo_sens; }
                if btn_dpad_right || merged_dpad_x > 0.5 { pan = servo_sens; }
                if btn_dpad_up || merged_dpad_y > 0.5 { tilt = servo_sens; } 
                if btn_dpad_down || merged_dpad_y < -0.5 { tilt = -servo_sens; }
                
                if merged_r_pan.abs() > 0.0 { pan = merged_r_pan * servo_sens; }
                if merged_r_tilt.abs() > 0.0 { tilt = merged_r_tilt * servo_sens; }

                pan = pan.clamp(-1.0, 1.0);
                tilt = tilt.clamp(-1.0, 1.0);

                if viewport_turn {
                    merged_steering = pan;
                }

                if merged_throttle != last_throttle || merged_steering != last_steering || pan != last_pan || tilt != last_tilt {
                    let mut cmd_map = serde_json::Map::new();
                    cmd_map.insert("type".to_string(), json!("command"));
                    cmd_map.insert("throttle".to_string(), json!(merged_throttle));
                    cmd_map.insert("steering".to_string(), json!(merged_steering));

                    if pan != last_pan || tilt != last_tilt {
                        cmd_map.insert("pan".to_string(), json!(pan));
                        cmd_map.insert("tilt".to_string(), json!(tilt));
                    }
                    
                    let cmd = Value::Object(cmd_map);

                    let state = app_handle.state::<WsState>();
                    let tx_lock = state.tx.blocking_lock();
                    if let Some(tx) = &*tx_lock {
                        let _ = tx.blocking_send(cmd.to_string());
                    }

                    last_throttle = merged_throttle;
                    last_steering = merged_steering;
                    last_pan = pan;
                    last_tilt = tilt;
                }
            }

            std::thread::sleep(Duration::from_millis(16));
        }
    });
}

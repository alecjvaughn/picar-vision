use crate::websocket::WsState;
use crate::ControlSettings;
use gilrs::{Axis, Button, Event, EventType, Gilrs};
use serde_json::{json, Value};
use std::sync::atomic::Ordering;
use std::time::Duration;
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

        let mut active_gamepad = None;
        let mut pan: f32 = 0.0;
        let mut tilt: f32 = 0.0;
        
        let mut last_throttle = 0.0;
        let mut last_steering = 0.0;
        let mut last_pan = 0.0;
        let mut last_tilt = 0.0;

        loop {
            // Examine new events
            while let Some(Event { id, event, .. }) = gilrs.next_event() {
                active_gamepad = Some(id);

                match event {
                    EventType::Connected => {
                        let _ = app_handle.emit(
                            "controller-status",
                            json!({"connected": true, "name": gilrs.gamepad(id).name()}),
                        );
                    }
                    EventType::Disconnected => {
                        active_gamepad = None;
                        let _ = app_handle.emit("controller-status", json!({"connected": false}));
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

            // If we have an active gamepad, poll its state for commands
            if let Some(id) = active_gamepad {
                let gamepad = gilrs.gamepad(id);

                let settings = app_handle.state::<ControlSettings>();
                let motor_scale = settings.motor_speed.load(Ordering::Relaxed) as f32 / 100.0;
                let servo_sens = settings.servo_sensitivity.load(Ordering::Relaxed) as f32 / 100.0;

                let mut throttle = gamepad.value(Axis::LeftStickY);
                let mut steering = gamepad.value(Axis::LeftStickX);
                
                let r_pan = gamepad.value(Axis::RightStickX);
                let r_tilt = gamepad.value(Axis::RightStickY);

                // Apply deadzone to prevent drift and runaway motors
                if throttle.abs() < 0.20 {
                    throttle = 0.0;
                }
                if steering.abs() < 0.20 {
                    steering = 0.0;
                }

                throttle *= motor_scale;
                steering *= motor_scale;

                // Handle continuous D-Pad and Right Stick servo movement
                let step = 0.016 * servo_sens; // max 1.6% per loop at 60Hz (-1.0 to 1.0 range)
                
                let dpad_x = gamepad.axis_data(Axis::DPadX).map(|a| a.value()).unwrap_or(0.0);
                let dpad_y = gamepad.axis_data(Axis::DPadY).map(|a| a.value()).unwrap_or(0.0);

                if gamepad.is_pressed(Button::DPadLeft) || dpad_x < -0.5 {
                    pan -= step;
                }
                if gamepad.is_pressed(Button::DPadRight) || dpad_x > 0.5 {
                    pan += step;
                }
                if gamepad.is_pressed(Button::DPadUp) || dpad_y > 0.5 {
                    tilt += step; // Inverted: UP is positive tilt
                }
                if gamepad.is_pressed(Button::DPadDown) || dpad_y < -0.5 {
                    tilt -= step;
                }
                
                // Add right joystick analog values if outside deadzone
                if r_pan.abs() > 0.20 {
                    pan += step * r_pan; // Right is positive pan
                }
                if r_tilt.abs() > 0.20 {
                    tilt += step * r_tilt; // Up is positive tilt (assuming Gilrs Y is positive UP)
                }

                // Clamp pan and tilt (-1.0 to 1.0 instead of degrees)
                pan = pan.clamp(-1.0, 1.0);
                tilt = tilt.clamp(-1.0, 1.0);

                if throttle != last_throttle || steering != last_steering || pan != last_pan || tilt != last_tilt {
                    // Send the command, omitting pan/tilt if they haven't changed
                    // This prevents the gamepad from clobbering the web UI's camera state when only driving
                    let mut cmd_map = serde_json::Map::new();
                    cmd_map.insert("type".to_string(), json!("command"));
                    cmd_map.insert("throttle".to_string(), json!(throttle));
                    cmd_map.insert("steering".to_string(), json!(steering));

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

                    last_throttle = throttle;
                    last_steering = steering;
                    last_pan = pan;
                    last_tilt = tilt;
                }
            }

            std::thread::sleep(Duration::from_millis(16));
        }
    });
}

use crate::websocket::WsState;
use crate::ControlSettings;
use gilrs::{Axis, Button, Event, EventType, Gilrs};
use serde_json::json;
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
                let mut steering = gamepad.value(Axis::RightStickX);

                // Apply deadzone to prevent drift and runaway motors
                if throttle.abs() < 0.20 {
                    throttle = 0.0;
                }
                if steering.abs() < 0.20 {
                    steering = 0.0;
                }

                throttle *= motor_scale;
                steering *= motor_scale;

                // Handle continuous D-Pad servo movement
                let step = 5.0 * servo_sens; // max 5 degrees per loop
                if gamepad.is_pressed(Button::DPadLeft) {
                    pan += step;
                }
                if gamepad.is_pressed(Button::DPadRight) {
                    pan -= step;
                }
                if gamepad.is_pressed(Button::DPadUp) {
                    tilt -= step;
                }
                if gamepad.is_pressed(Button::DPadDown) {
                    tilt += step;
                }

                // Clamp pan and tilt (-90 to 90 degrees typically)
                pan = pan.clamp(-90.0, 90.0);
                tilt = tilt.clamp(-90.0, 90.0);

                // Send the command
                let cmd = json!({
                    "type": "command",
                    "throttle": throttle,
                    "steering": steering,
                    "pan": pan,
                    "tilt": tilt
                });

                let state = app_handle.state::<WsState>();
                let tx_lock = state.tx.blocking_lock();
                if let Some(tx) = &*tx_lock {
                    let _ = tx.blocking_send(cmd.to_string());
                }
            }

            std::thread::sleep(Duration::from_millis(50));
        }
    });
}

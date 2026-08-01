use gilrs::{Axis, Event, EventType, Gilrs};
use serde_json::json;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use crate::websocket::WsState;

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

        loop {
            // Examine new events
            while let Some(Event { id, event, time: _, .. }) = gilrs.next_event() {
                active_gamepad = Some(id);
                
                match event {
                    EventType::Connected => {
                        let _ = app_handle.emit("controller-status", json!({"connected": true, "name": gilrs.gamepad(id).name()}));
                    }
                    EventType::Disconnected => {
                        active_gamepad = None;
                        let _ = app_handle.emit("controller-status", json!({"connected": false}));
                    }
                    _ => {}
                }
            }

            // If we have an active gamepad, poll its state for commands
            if let Some(id) = active_gamepad {
                let gamepad = gilrs.gamepad(id);
                
                // Read left stick Y for throttle (-1.0 to 1.0)
                let throttle = gamepad.value(Axis::LeftStickY);
                // Read right stick X for steering (-1.0 to 1.0)
                let steering = gamepad.value(Axis::RightStickX);
                
                // Example: map throttle to speed 0-100 and direction
                // You can customize the mapping here
                
                // Send the command if connected
                if throttle.abs() > 0.05 || steering.abs() > 0.05 {
                    let cmd = json!({
                        "type": "command",
                        "throttle": throttle,
                        "steering": steering
                    });
                    
                    let state = app_handle.state::<WsState>();
                    let tx_lock = state.tx.blocking_lock();
                    if let Some(tx) = &*tx_lock {
                        let _ = tx.blocking_send(cmd.to_string());
                    }
                }
            }

            std::thread::sleep(Duration::from_millis(50));
        }
    });
}

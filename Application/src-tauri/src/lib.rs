pub mod controller;
pub mod websocket;

use std::sync::atomic::{AtomicU32, AtomicBool, Ordering};
use tokio::sync::Mutex;
use websocket::{connect_to_pi, disconnect_from_pi, send_pi_command, WsState};

pub struct ControlSettings {
    pub motor_speed: AtomicU32,
    pub servo_sensitivity: AtomicU32,
    pub follow_camera: AtomicBool,
}

#[tauri::command]
fn update_settings(
    motor_speed: u32,
    servo_sensitivity: u32,
    follow_camera: bool,
    settings: tauri::State<ControlSettings>,
) {
    settings.motor_speed.store(motor_speed, Ordering::Relaxed);
    settings
        .servo_sensitivity
        .store(servo_sensitivity, Ordering::Relaxed);
    settings
        .follow_camera
        .store(follow_camera, Ordering::Relaxed);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(WsState {
            tx: Mutex::new(None),
        })
        .manage(ControlSettings {
            motor_speed: AtomicU32::new(100),
            servo_sensitivity: AtomicU32::new(50),
            follow_camera: AtomicBool::new(false),
        })
        .invoke_handler(tauri::generate_handler![
            connect_to_pi,
            send_pi_command,
            disconnect_from_pi,
            update_settings
        ])
        .setup(|app| {
            let handle = app.handle().clone();
            controller::start_controller_loop(handle);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

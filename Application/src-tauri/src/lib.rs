pub mod controller;
pub mod websocket;

use std::sync::atomic::{AtomicI32, AtomicU32, AtomicBool, Ordering};
use tokio::sync::Mutex;
use websocket::{connect_to_pi, disconnect_from_pi, send_pi_command, WsState};

pub struct ControlSettings {
    pub motor_speed: AtomicU32,
    pub servo_sensitivity: AtomicU32,
    pub viewport_turn: AtomicBool,
    
    // Calibration parameters (values * 1000, so 0.150 is 150)
    pub lx_center: AtomicI32,
    pub ly_center: AtomicI32,
    pub rx_center: AtomicI32,
    pub ry_center: AtomicI32,
    
    pub lx_deadzone: AtomicI32,
    pub ly_deadzone: AtomicI32,
    pub rx_deadzone: AtomicI32,
    pub ry_deadzone: AtomicI32,
}

#[tauri::command]
fn update_settings(
    motor_speed: u32,
    servo_sensitivity: u32,
    viewport_turn: bool,
    settings: tauri::State<ControlSettings>,
) {
    settings.motor_speed.store(motor_speed, Ordering::Relaxed);
    settings
        .servo_sensitivity
        .store(servo_sensitivity, Ordering::Relaxed);
    settings
        .viewport_turn
        .store(viewport_turn, Ordering::Relaxed);
}

#[tauri::command]
fn update_calibration(
    lx_c: i32, ly_c: i32, rx_c: i32, ry_c: i32,
    lx_d: i32, ly_d: i32, rx_d: i32, ry_d: i32,
    settings: tauri::State<ControlSettings>,
) {
    settings.lx_center.store(lx_c, Ordering::Relaxed);
    settings.ly_center.store(ly_c, Ordering::Relaxed);
    settings.rx_center.store(rx_c, Ordering::Relaxed);
    settings.ry_center.store(ry_c, Ordering::Relaxed);
    
    settings.lx_deadzone.store(lx_d, Ordering::Relaxed);
    settings.ly_deadzone.store(ly_d, Ordering::Relaxed);
    settings.rx_deadzone.store(rx_d, Ordering::Relaxed);
    settings.ry_deadzone.store(ry_d, Ordering::Relaxed);
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
            viewport_turn: AtomicBool::new(false),
            lx_center: AtomicI32::new(0),
            ly_center: AtomicI32::new(0),
            rx_center: AtomicI32::new(0),
            ry_center: AtomicI32::new(0),
            lx_deadzone: AtomicI32::new(200), // 0.20 deadzone
            ly_deadzone: AtomicI32::new(200),
            rx_deadzone: AtomicI32::new(200),
            ry_deadzone: AtomicI32::new(200),
        })
        .invoke_handler(tauri::generate_handler![
            connect_to_pi,
            send_pi_command,
            disconnect_from_pi,
            update_settings,
            update_calibration
        ])
        .setup(|app| {
            let handle = app.handle().clone();
            controller::start_controller_loop(handle);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

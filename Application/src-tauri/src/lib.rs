pub mod websocket;
pub mod controller;

use tokio::sync::Mutex;
use websocket::{WsState, connect_to_pi, send_pi_command};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(WsState { tx: Mutex::new(None) })
        .invoke_handler(tauri::generate_handler![connect_to_pi, send_pi_command])
        .setup(|app| {
            let handle = app.handle().clone();
            controller::start_controller_loop(handle);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

pub struct WsState {
    pub tx: Mutex<Option<mpsc::Sender<String>>>,
}

#[tauri::command]
pub async fn connect_to_pi(ip: String, app: AppHandle, state: State<'_, WsState>) -> Result<(), String> {
    let url = format!("ws://{}:8765", ip);

    let (ws_stream, _) = connect_async(&url).await.map_err(|e| e.to_string())?;
    let (mut write, mut read) = ws_stream.split();

    // Create a channel for sending messages out to the websocket
    let (tx, mut rx) = mpsc::channel::<String>(32);
    *state.tx.lock().await = Some(tx);

    // Task for sending messages
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if write.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Task for reading messages
    tokio::spawn(async move {
        app.emit("ws-connected", ()).unwrap();
        
        while let Some(Ok(msg)) = read.next().await {
            match msg {
                Message::Text(text) => {
                    // Try parsing to see what it is
                    if let Ok(json) = serde_json::from_str::<Value>(&text) {
                        if json.get("type").and_then(|t| t.as_str()) == Some("telemetry") {
                            let _ = app.emit("telemetry", json);
                        } else {
                            let _ = app.emit("json-message", json);
                        }
                    }
                }
                Message::Binary(bin) => {
                    // Assuming binary messages are MJPEG frames
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

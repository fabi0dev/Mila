use tauri::{AppHandle, Emitter};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub enum AppState {
    WakeWordDetected,
    SttResult(String),
}

pub fn emit_state(app: &AppHandle, state: AppState) {
    let (event_name, payload) = match state {
        AppState::WakeWordDetected => ("WAKE_WORD_DETECTED", serde_json::Value::Null),
        AppState::SttResult(res) => ("STT_RESULT", serde_json::json!(res)),
    };
    app.emit(event_name, payload).unwrap();
}

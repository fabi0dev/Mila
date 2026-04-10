use tauri::AppHandle;
use crate::modules::audio::{emit_state, AppState};

#[tauri::command]
pub fn test_wake_word(app: AppHandle) {
    emit_state(&app, AppState::WakeWordDetected);
}

#[tauri::command]
pub fn test_stt(app: AppHandle, result: String) {
    emit_state(&app, AppState::SttResult(result));
}

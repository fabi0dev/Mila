mod modules;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .setup(|app| {
      // Load environment variables
      dotenvy::dotenv().ok();

      // Initialize and start Audio Engine
      let app_handle = app.handle().clone();
      let audio_engine = crate::modules::audio_engine::AudioEngine::new(app_handle);
      if let Err(e) = audio_engine.start() {
        eprintln!("Failed to start Audio Engine: {:?}", e);
      }

      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      modules::commands::test_wake_word,
      modules::commands::test_stt
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

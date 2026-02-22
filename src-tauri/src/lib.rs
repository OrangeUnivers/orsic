mod audio;
use audio::{get_metadata, pause_audio, play_audio, resume_audio, start_audio_thread, stop_audio};
use std::sync::Mutex;

pub fn app() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(start_audio_thread()))
        .invoke_handler(tauri::generate_handler![
            play_audio,
            pause_audio,
            resume_audio,
            stop_audio,
            get_metadata
        ])
        .run(tauri::generate_context!())
        .expect("error running app");
}

#[cfg(mobile)]
#[tauri::mobile_entry_point]
fn main() {
    app();
}

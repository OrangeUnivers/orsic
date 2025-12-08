mod audio;
use audio::{start_audio_thread, AudioCommand};
use std::sync::Mutex;

#[tauri::command]
fn play_audio(tx: tauri::State<Mutex<std::sync::mpsc::Sender<AudioCommand>>>, path: String) {
    tx.lock().unwrap().send(AudioCommand::Play(path)).unwrap();
}

#[tauri::command]
fn pause_audio(tx: tauri::State<Mutex<std::sync::mpsc::Sender<AudioCommand>>>) {
    tx.lock().unwrap().send(AudioCommand::Pause).unwrap();
}

#[tauri::command]
fn resume_audio(tx: tauri::State<Mutex<std::sync::mpsc::Sender<AudioCommand>>>) {
    tx.lock().unwrap().send(AudioCommand::Resume).unwrap();
}

#[tauri::command]
fn stop_audio(tx: tauri::State<Mutex<std::sync::mpsc::Sender<AudioCommand>>>) {
    tx.lock().unwrap().send(AudioCommand::Stop).unwrap();
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(start_audio_thread()))
        .invoke_handler(tauri::generate_handler![
            play_audio,
            pause_audio,
            resume_audio,
            stop_audio
        ])
        .run(tauri::generate_context!())
        .expect("error running app");
}

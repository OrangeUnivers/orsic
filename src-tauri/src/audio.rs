use rodio::{Decoder, OutputStream, Sink};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;
use std::{fs::File, io::BufReader};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::get_probe;

#[derive(serde::Serialize)]
pub struct TrackMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration_ms: Option<u64>,
    pub cover_art: Option<String>, // reserved for future (e.g. PNG next to file)
}

#[tauri::command]
pub fn get_metadata(path: String) -> Result<TrackMetadata, String> {
    let file = File::open(&path).map_err(|e| e.to_string())?;
    let hint = Hint::new();
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    let probe = get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .map_err(|e| e.to_string())?;

    let mut format = probe.format;

    let mut title = None;
    let mut artist = None;
    let mut album = None;
    let mut duration_ms = None;

    let title_keys = ["title", "TIT2", "©nam"];
    let artist_keys = ["artist", "TPE1", "©ART"];
    let album_keys = ["album", "TALB", "©alb"];

    if let Some(metadata) = format.metadata().current() {
        for tag in metadata.tags() {
            let key = tag.key.to_lowercase();
            let value = tag.value.to_string();

            if title_keys.iter().any(|k| k.eq_ignore_ascii_case(&key)) {
                title = Some(value);
            } else if artist_keys.iter().any(|k| k.eq_ignore_ascii_case(&key)) {
                artist = Some(value);
            } else if album_keys.iter().any(|k| k.eq_ignore_ascii_case(&key)) {
                album = Some(value);
            }
        }
    }

    if let Some(track) = format.tracks().first() {
        if let Some(timebase) = track.codec_params.time_base {
            if let Some(n_frames) = track.codec_params.n_frames {
                let time = timebase.calc_time(n_frames);
                duration_ms = Some(time.seconds * 1000);
            }
        }
    }

    Ok(TrackMetadata {
        title,
        artist,
        album,
        duration_ms,
        cover_art: None,
    })
}

pub enum AudioCommand {
    Play(String),
    Pause,
    Resume,
    Stop,
}

pub fn start_audio_thread() -> Sender<AudioCommand> {
    let (tx, rx): (Sender<AudioCommand>, Receiver<AudioCommand>) = channel();

    std::thread::spawn(move || {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let mut sink: Option<Sink> = None;

        while let Ok(cmd) = rx.recv() {
            match cmd {
                AudioCommand::Play(path) => {
                    let file = File::open(path).unwrap();
                    let source = Decoder::new(BufReader::new(file)).unwrap();
                    let new_sink = Sink::try_new(&handle).unwrap();
                    new_sink.append(source);
                    new_sink.play();
                    sink = Some(new_sink);
                }
                AudioCommand::Pause => {
                    if let Some(ref s) = sink {
                        s.pause();
                    }
                }
                AudioCommand::Resume => {
                    if let Some(ref s) = sink {
                        s.play();
                    }
                }
                AudioCommand::Stop => {
                    sink = None;
                }
            }
        }
    });

    tx
}

#[tauri::command]
pub fn play_audio(tx: tauri::State<Mutex<Sender<AudioCommand>>>, path: String) {
    tx.lock().unwrap().send(AudioCommand::Play(path)).unwrap();
}

#[tauri::command]
pub fn pause_audio(tx: tauri::State<Mutex<Sender<AudioCommand>>>) {
    tx.lock().unwrap().send(AudioCommand::Pause).unwrap();
}

#[tauri::command]
pub fn resume_audio(tx: tauri::State<Mutex<Sender<AudioCommand>>>) {
    tx.lock().unwrap().send(AudioCommand::Resume).unwrap();
}

#[tauri::command]
pub fn stop_audio(tx: tauri::State<Mutex<Sender<AudioCommand>>>) {
    tx.lock().unwrap().send(AudioCommand::Stop).unwrap();
}

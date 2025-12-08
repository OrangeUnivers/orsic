use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader};
use std::sync::mpsc::{Sender, Receiver, channel};

pub enum AudioCommand {
    Play(String),
    Pause,
    Resume,
    Stop,
}

pub fn start_audio_thread() -> Sender<AudioCommand> {
    let (tx, rx): (Sender<AudioCommand>, Receiver<AudioCommand>) = channel();

    std::thread::spawn(move || {
        // Rodio objects live here (in this thread only)
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

mod audio;

use rodio::{Source, Device, Sink};
use std::io::Cursor;
pub use crate::audio::Audio;
use crate::audio::AudioSink;

// const FILE: &'static [u8] = include_bytes!("assets/engine3.ogg");

pub struct AudioContext {
    device: Device,
}

impl AudioContext {
    pub fn new() -> Result<Self, String> {
        let device = rodio::default_output_device()
            .ok_or("Cannot find audio output device.")?;
        Ok(Self {
            device
        })
    }

    pub fn play(&mut self, audio: &mut Audio) {
        // rodio::play_raw(&self.device, audio.decoder().unwrap());
        // let samples = audio.decoder().unwrap().convert_samples();
        // rodio::play_raw(&self.device, samples);
        // let sink = AudioSink {
        //     sink: Sink::new(&self.device)
        // };
        //
        // sink.sink.append(audio.decoder().unwrap());
        //
        // sink.sink.play();
        audio.play(&self);
    }

    pub fn stop(&mut self, audio: &mut Audio) {
        audio.stop(&self);
    }
}

// pub fn test_audio() {
//     let device = rodio::default_output_device().unwrap();
//     let source = rodio::Decoder::new(Cursor::new(FILE)).unwrap();
//     rodio::play_raw(&device, source.convert_samples());
// }
mod audio;

use crate::audio::decoder;
pub use crate::audio::{Audio, Sound};
use rodio::{Device, Sink, Source};
use std::io::Cursor;

// const FILE: &'static [u8] = include_bytes!("assets/engine3.ogg");

pub struct AudioContext {
    device: Device,
}

impl AudioContext {
    pub fn new() -> Result<Self, String> {
        let device = rodio::default_output_device().ok_or("Cannot find audio output device.")?;
        Ok(Self { device })
    }

    pub fn sound(&mut self, audio: &Audio) -> Result<Sound, String> {
        Sound::new(&self, audio)
    }

    pub fn play(&mut self, audio: &Audio) {
        rodio::play_raw(
            &self.device,
            decoder(audio.source.clone()).unwrap().convert_samples(),
        );
    }
}

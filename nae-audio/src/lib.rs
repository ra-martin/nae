mod audio;

use crate::audio::decoder;
pub use crate::audio::{Audio, AudioSource};
use rodio::{Device, Sink, Source};
use std::io::Cursor;

// const FILE: &'static [u8] = include_bytes!("assets/engine3.ogg");

pub struct AudioContext {
    device: std::sync::Arc<Device>,
}

impl AudioContext {
    pub fn new() -> Result<Self, String> {
        let device = std::sync::Arc::new(
            rodio::default_output_device().ok_or("Cannot find audio output device.")?,
        );
        Ok(Self { device })
    }

    /// Returns a audio instance
    pub fn instance(&mut self, source: &AudioSource) -> Audio {
        Audio::new(&self, source)
    }

    /// Play a sound until it finish
    pub fn play(&mut self, source: &AudioSource) {
        rodio::play_raw(
            &self.device,
            decoder(source.source.clone()).unwrap().convert_samples(),
        );
    }

    pub fn volume(&self) -> f32 {
        unimplemented!()
    }

    pub fn set_volume(&mut self) {
        unimplemented!()
    }

    //TODO stop all?
}

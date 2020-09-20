mod audio;

use crate::audio::InnerAudio;
use crate::audio::{clamp, decoder};
pub use crate::audio::{Audio, AudioSource};
use rodio::{Device, Sink, Source};
use std::io::Cursor;
use std::sync::{Arc, Weak};

// const FILE: &'static [u8] = include_bytes!("assets/engine3.ogg");

pub struct AudioContext {
    device: std::sync::Arc<Device>,
    pub(crate) current_audios: Vec<Weak<InnerAudio>>,
    volume: f32,
}

// track audio and sound? sound need to be tracked to change the volume if the master gain change even if we don't have the control

impl AudioContext {
    pub fn new() -> Result<Self, String> {
        let device = std::sync::Arc::new(
            rodio::default_output_device().ok_or("Cannot find audio output device.")?,
        );
        Ok(Self {
            device,
            current_audios: vec![],
            volume: 1.0,
        })
    }

    /// Returns a audio instance
    pub fn instance(&mut self, source: &AudioSource) -> Audio {
        let audio = Audio::new(&self, source);
        self.current_audios.push(Arc::downgrade(&audio.inner));
        audio
    }

    /// Play a sound until it finish
    pub fn play(&mut self, source: &AudioSource) {
        rodio::play_raw(
            &self.device,
            decoder(source.source.clone()).unwrap().convert_samples(),
        );
    }

    pub fn tick(&mut self) {
        //pub crate? private?
        self.current_audios = self
            .current_audios
            .drain(..)
            .filter(|weak_audio| weak_audio.upgrade().is_some())
            .collect();
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, value: f32) {
        let volume = clamp(value, 0.0, 1.0);

        self.volume = volume;
        self.current_audios.iter_mut().for_each(|weak_audio| {
            if let Some(audio) = weak_audio.upgrade() {
                audio.update_global_volume(volume);
            }
        });
    }

    //TODO stop all?
}

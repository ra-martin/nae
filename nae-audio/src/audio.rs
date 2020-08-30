use crate::AudioContext;
use nae_core::{BaseApp, BaseSystem};
use rodio::decoder::{Decoder, DecoderError};
use rodio::source::{Buffered, SamplesConverter};
use rodio::{Device, Sink, Source};
use std::io::Cursor;
use std::sync::Arc;

pub struct AudioSource {
    pub(crate) source: Arc<[u8]>,
}

impl AudioSource {
    /// Create a new texture from bytes
    pub fn from_bytes<T, S>(app: &mut T, data: &[u8]) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem,
    {
        // returns an error if the data cannot be used as a valid audio
        let _ = Decoder::new(Cursor::new(data.to_vec())).map_err(|err| err.to_string())?;

        let source = Arc::from(data);
        Ok(Self { source })
    }
}

pub struct Audio {
    source: Arc<[u8]>,
    sink: Option<Sink>,
    device: Arc<Device>,

    volume: f32,
}

impl Audio {
    pub(crate) fn new(ctx: &AudioContext, audio: &AudioSource) -> Self {
        let source = audio.source.clone();

        Self {
            source,
            device: ctx.device.clone(),
            sink: None,
            volume: 1.0,
        }
    }

    /// Play or resume the audio
    pub fn play(&mut self) {
        if self.is_playing() {
            return;
        }

        if self.is_paused() {
            if let Some(sink) = &self.sink {
                sink.play();
            }

            return;
        }

        let sink = Sink::new(&self.device);
        sink.append(decoder(self.source.clone()).unwrap());
        sink.set_volume(self.volume);
        sink.play();
        self.sink = Some(sink);
    }

    /// Stops the audio
    pub fn stop(&mut self) {
        if let Some(sink) = &self.sink.take() {
            sink.stop();
        }
    }

    /// Returns if the audio is currently playing
    pub fn is_playing(&self) -> bool {
        self.sink.as_ref().map_or(false, |sink| sink.len() != 0)
    }

    /// Pause the audio
    pub fn pause(&mut self) {
        if let Some(sink) = &self.sink {
            sink.pause();
        }
    }

    /// Returns if the audio is paused
    pub fn is_paused(&self) -> bool {
        self.sink.as_ref().map_or(false, |sink| sink.is_paused())
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, value: f32) {
        self.volume = clamp(value, 0.0, 1.0);
        if let Some(sink) = &self.sink {
            sink.set_volume(self.volume);
        }
    }

    //TODO loop, repeat, etc...
}

#[inline]
pub(crate) fn decoder(source: Arc<[u8]>) -> Result<Decoder<Cursor<Arc<[u8]>>>, String> {
    Decoder::new(Cursor::new(source.clone())).map_err(|err| err.to_string())
}

#[inline]
fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

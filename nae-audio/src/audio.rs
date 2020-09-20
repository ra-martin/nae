use crate::AudioContext;
use nae_core::{BaseApp, BaseSystem};
use rodio::decoder::{Decoder, DecoderError};
use rodio::source::{Buffered, SamplesConverter};
use rodio::{Device, Sink, Source};
use std::io::Cursor;
use std::sync::{Arc, RwLock};

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

pub(crate) struct AudioState {
    volume: RwLock<f32>,
    global_volume: RwLock<f32>,
}

impl AudioState {
    fn new(ctx: &AudioContext) -> Self {
        let volume = RwLock::new(1.0);
        let global_volume = RwLock::new(ctx.volume());
        Self {
            volume,
            global_volume,
        }
    }

    fn volume(&self) -> f32 {
        *self.volume.read().unwrap()
    }

    fn set_volume(&self, value: f32) {
        *self.volume.write().unwrap() = value;
    }

    fn set_global_volume(&self, value: f32) {
        *self.global_volume.write().unwrap() = value;
    }

    pub fn global_volume(&self) -> f32 {
        *self.global_volume.read().unwrap()
    }
}

pub(crate) struct InnerAudio {
    source: Arc<[u8]>,
    sink: RwLock<Option<Sink>>,
    device: Arc<Device>,
    state: AudioState,
}

impl InnerAudio {
    fn new(ctx: &AudioContext, audio: &AudioSource) -> Self {
        let source = audio.source.clone();
        Self {
            source,
            device: ctx.device.clone(),
            sink: RwLock::new(None),
            state: AudioState::new(ctx),
        }
    }

    fn play(&self) {
        if self.is_playing() {
            return;
        }

        if self.is_paused() {
            if let Some(sink) = &*self.sink.read().unwrap() {
                sink.play();
            }

            return;
        }

        let sink = Sink::new(&self.device);
        let decoder = decoder(self.source.clone()).unwrap();
        sink.append(decoder);
        sink.set_volume(self.state.volume() * self.state.global_volume());
        sink.play();
        *self.sink.write().unwrap() = Some(sink);
    }

    fn stop(&self) {
        if let Some(sink) = &self.sink.write().unwrap().take() {
            sink.stop();
        }
    }

    fn is_playing(&self) -> bool {
        self.sink
            .read()
            .unwrap()
            .as_ref()
            .map_or(false, |sink| sink.len() != 0)
    }

    fn toggle_play(&self) {
        match self.is_playing() {
            true => self.stop(),
            false => self.play(),
        };
    }

    fn pause(&self) {
        if let Some(sink) = &*self.sink.read().unwrap() {
            sink.pause();
        }
    }

    fn is_paused(&self) -> bool {
        self.sink
            .read()
            .unwrap()
            .as_ref()
            .map_or(false, |sink| sink.is_paused())
    }

    fn volume(&self) -> f32 {
        self.state.volume()
    }

    fn set_volume(&self, value: f32) {
        let volume = clamp(value, 0.0, 1.0);
        self.state.set_volume(volume);

        let global_volume = self.state.global_volume();
        if let Some(sink) = &*self.sink.read().unwrap() {
            sink.set_volume(volume * global_volume);
        }
    }

    pub fn update_global_volume(&self, value: f32) {
        self.state.set_global_volume(value);
        if let Some(sink) = &*self.sink.read().unwrap() {
            sink.set_volume(self.volume() * value);
        }
    }

    fn repeat(&self) {
        //TODO?
    }

    //TODO loop, repeat, etc...
}

#[derive(Clone)]
pub struct Audio {
    pub(crate) inner: Arc<InnerAudio>,
}

impl Audio {
    pub(crate) fn new(ctx: &AudioContext, audio: &AudioSource) -> Self {
        Self {
            inner: Arc::new(InnerAudio::new(ctx, audio)),
        }
    }

    /// Play or resume the audio
    #[inline]
    pub fn play(&mut self) {
        self.inner.play();
    }

    /// Stops the audio
    #[inline]
    pub fn stop(&mut self) {
        self.inner.stop();
    }

    /// Returns if the audio is currently playing
    #[inline]
    pub fn is_playing(&self) -> bool {
        self.inner.is_playing()
    }

    /// Stop the audio if it's playing or play it if it's stopped
    #[inline]
    pub fn toggle_play(&mut self) {
        self.inner.toggle_play();
    }

    /// Pause the audio
    #[inline]
    pub fn pause(&mut self) {
        self.inner.pause();
    }

    /// Returns if the audio is paused
    #[inline]
    pub fn is_paused(&self) -> bool {
        self.inner.is_paused()
    }

    /// Returns the volume value (between 0.0 and 1.0)
    #[inline]
    pub fn volume(&self) -> f32 {
        self.inner.volume()
    }

    /// Set the volume for this instance
    #[inline]
    pub fn set_volume(&mut self, value: f32) {
        self.inner.set_volume(value);
    }
}

#[inline]
pub(crate) fn decoder(source: Arc<[u8]>) -> Result<Decoder<Cursor<Arc<[u8]>>>, String> {
    Decoder::new(Cursor::new(source.clone())).map_err(|err| err.to_string())
}

#[inline]
pub(crate) fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

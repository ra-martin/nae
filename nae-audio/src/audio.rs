use crate::AudioContext;
use nae_core::{BaseApp, BaseSystem};
use rodio::decoder::{Decoder, DecoderError};
use rodio::source::{Buffered, SamplesConverter};
use rodio::{Device, Sink, Source};
use std::io::Cursor;
use std::sync::Arc;

pub struct Audio {
    pub(crate) source: Arc<[u8]>,
}

impl Audio {
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

pub struct Sound {
    source: Arc<[u8]>,
    sink: Sink,

    playing: bool,
}

impl Sound {
    pub(crate) fn new(ctx: &AudioContext, audio: &Audio) -> Result<Self, String> {
        let sink = Sink::new(&ctx.device);
        let source = audio.source.clone();

        Ok(Self {
            source,
            sink,

            playing: false,
        })
    }

    pub fn play(&mut self) {
        if self.playing {
            return;
        }
        self.sink.append(decoder(self.source.clone()).unwrap());
        self.sink.play();
    }

    pub fn stop(&mut self) {
        self.sink.stop();
        self.playing = false;
    }
}

pub(crate) fn decoder(source: Arc<[u8]>) -> Result<Decoder<Cursor<Arc<[u8]>>>, String> {
    Decoder::new(Cursor::new(source.clone())).map_err(|err| err.to_string())
}

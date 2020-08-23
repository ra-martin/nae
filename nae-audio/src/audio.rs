use rodio::decoder::{Decoder, DecoderError};
use std::io::Cursor;
use nae_core::{BaseApp, BaseSystem};
use rodio::{Source, Sink, Device};
use rodio::source::{Buffered, SamplesConverter};
use std::sync::Arc;
use crate::AudioContext;

pub struct Audio {
    pub(crate) source: Arc<[u8]>,
    sink: Option<Sink>,
}

impl Audio {
    // pub fn is_loaded(&self) -> bool {
    //
    // }

    /// Create a new texture from bytes
    pub fn from_bytes<T, S>(app: &mut T, data: &[u8]) -> Result<Self, String>
        where
            T: BaseApp<System = S>,
            S: BaseSystem,
    {
        // if it's invalid data to be used as sound return err
        let _ = Decoder::new(Cursor::new(data.to_vec()))
            .map_err(|err| err.to_string())?;

        let source = Arc::from(data);
        Ok(Self {
            source,
            sink: None,
        })
    }

    pub fn play(&mut self, ctx: &AudioContext) {
        if self.sink.is_none() {
            self.sink = Some(Sink::new(&ctx.device));
        }

        if let Some(sink) = &self.sink {
            sink.append(self.decoder().unwrap());
            sink.play();
        }
    }

    pub fn stop(&mut self, ctx: &AudioContext) {
        if let Some(sink) = &self.sink {
            sink.stop();
        }
    }

    pub(crate) fn decoder(&self) -> Result<Decoder<Cursor<Arc<[u8]>>>, String> {
        rodio::Decoder::new(Cursor::new(self.source.clone()))
            .map_err(|err| err.to_string())
    }
}

pub struct AudioSink {
    pub sink: Sink,
}


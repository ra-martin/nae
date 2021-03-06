use super::ResourceParser;
use crate::app::App;
use backend::load_file;
use backend::{Graphics, System};
use futures::{Async, Future};
use nae_core::BaseSystem;
use nae_core::*;

type ResourceLoader = (
    Box<dyn ResourceParser<App = App>>,
    Box<dyn Future<Item = Vec<u8>, Error = String>>,
);

pub(crate) struct ResourceLoaderManager {
    to_load: Vec<ResourceLoader>,
}

impl ResourceLoaderManager {
    pub fn new() -> Self {
        Self { to_load: vec![] }
    }

    pub fn len(&self) -> usize {
        self.to_load.len()
    }

    pub fn add<T>(&mut self, file: &str, resource: Box<T>) -> Result<(), String>
    where
        T: Resource<App> + ResourceParser<App = App> + 'static,
    {
        let fut = load_file(file);
        self.to_load.push((resource, Box::new(fut)));
        Ok(())
    }

    pub fn try_load(
        &mut self,
    ) -> Result<Option<Vec<(Vec<u8>, Box<dyn ResourceParser<App = App>>)>>, String> {
        if self.to_load.len() == 0 {
            return Ok(None);
        }

        let mut loaded = vec![];
        let mut not_loaded = vec![];

        while let Some(mut asset_loader) = self.to_load.pop() {
            if let AssetState::Done(data) = try_load_asset(&mut asset_loader)? {
                loaded.push((data, asset_loader.0));
            } else {
                not_loaded.push(asset_loader);
            }
        }

        self.to_load = not_loaded;

        Ok(Some(loaded))
    }
}

#[derive(Eq, PartialEq)]
pub(crate) enum AssetState {
    OnProgress,
    AlreadyLoaded,
    Done(Vec<u8>),
}

fn try_load_asset(loader: &mut ResourceLoader) -> Result<AssetState, String> {
    let (_, ref mut future) = loader;
    return future.poll().map(|s| {
        if let Async::Ready(buff) = s {
            AssetState::Done(buff)
        } else {
            AssetState::OnProgress
        }
    });
}

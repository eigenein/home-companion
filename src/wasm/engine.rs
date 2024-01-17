use std::path::Path;

use wasmtime::{Config, Store};

use crate::{
    prelude::*,
    wasm::{linker::Linker, module::Module},
};

/// WASM engine/linker wrapper.
pub struct Engine(wasmtime::Engine);

impl From<wasmtime::Engine> for Engine {
    fn from(inner: wasmtime::Engine) -> Self {
        Self(inner)
    }
}

impl Engine {
    pub fn new_async() -> Result<Self> {
        let mut config = Config::new();
        config.async_support(true);
        wasmtime::Engine::new(&config)
            .context("failed to create a WASM engine")
            .map(Into::into)
    }

    pub fn new_store<T>(&self, data: T) -> Store<T> {
        Store::new(&self.0, data)
    }

    pub fn new_linker<S>(&self) -> Linker<S> {
        wasmtime::Linker::<S>::new(&self.0).into()
    }

    pub fn load_module(&self, path: &Path) -> Result<Module> {
        info!(?path, "loading module…");
        wasmtime::Module::from_file(&self.0, path)
            .with_context(|| format!("failed to load WASM module from {path:?}"))
            .map(Into::into)
    }
}

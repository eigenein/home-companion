use std::path::Path;

use wasmtime::{Config, Store};

use crate::{
    companion::state::HostInstanceState,
    prelude::*,
    wasm::{linker::Linker, module::Module},
};

/// WASM engine/linker wrapper.
#[derive(derive_more::From, derive_more::AsMut)]
#[must_use]
pub struct Engine(wasmtime::Engine);

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

    pub fn new_linker<D: Send>(&self) -> Linker<HostInstanceState<D>> {
        Linker::from(wasmtime::Linker::new(&self.0))
    }

    pub fn load_module(&self, path: &Path) -> Result<Module> {
        wasmtime::Module::from_file(&self.0, path)
            .with_context(|| format!("failed to load WASM module from {path:?}"))
            .map(Module::from)
    }
}

use std::path::Path;

use wasmtime::{Caller, Config, Store};

use crate::{
    prelude::*,
    wasm::{linker::Linker, memory::Memory, module::Module},
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

    pub fn new_linker<D>(&self) -> Result<Linker<D>> {
        let mut linker = wasmtime::Linker::<D>::new(&self.0);
        linker.func_wrap(
            "logging",
            "info",
            |mut caller: Caller<'_, D>, offset: u32, size: u32| {
                info!(offset, size, "called `logging.info`");
                Memory::from_export(|name| caller.get_export(name))?;
                Ok(())
            },
        )?;
        Ok(linker.into())
    }

    pub fn load_module(&self, path: &Path) -> Result<Module> {
        wasmtime::Module::from_file(&self.0, path)
            .with_context(|| format!("failed to load WASM module from {path:?}"))
            .map(Into::into)
    }
}

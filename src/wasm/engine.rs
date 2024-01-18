use std::path::Path;

use tracing::Level;
use wasmtime::{AsContext, Caller, Config, Store};

use crate::{
    prelude::*,
    wasm::{
        linker::Linker,
        memory::{Memory, Segment},
        module::Module,
    },
};

/// WASM engine/linker wrapper.
#[derive(derive_more::From)]
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

    pub fn new_linker<D: Send>(&self) -> Result<Linker<D>> {
        let mut linker = wasmtime::Linker::<D>::new(&self.0);
        linker.func_wrap(
            "logging",
            "info",
            |mut caller: Caller<'_, D>, offset: u32, size: u32| {
                let message = Memory::try_from_caller(&mut caller)?
                    .read_bytes(caller.as_context(), Segment::try_from_u32(offset, size)?)?;
                event!(Level::INFO, "{}", String::from_utf8(message)?);
                Ok(())
            },
        )?;
        Ok(linker.into())
    }

    pub fn load_module(&self, path: &Path) -> Result<Module> {
        wasmtime::Module::from_file(&self.0, path)
            .with_context(|| format!("failed to load WASM module from {path:?}"))
            .map(Module::from)
    }
}

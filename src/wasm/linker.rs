use wasmtime::{AsContextMut, Caller};

use crate::{
    prelude::{instrument, *},
    wasm::{instance::Instance, memory::Memory, module::Module},
};

pub struct Linker<T = ()>(wasmtime::Linker<T>);

impl<D: Send> Linker<D> {
    /// Create and initialize a new linker.
    pub fn new(mut inner: wasmtime::Linker<D>) -> Result<Self> {
        inner.func_wrap(
            "logging",
            "error",
            |mut caller: Caller<'_, D>, offset: u32, size: u32| {
                error!("{}", Memory::read_string_from_caller(&mut caller, offset, size)?);
                Ok(())
            },
        )?;
        inner.func_wrap(
            "logging",
            "info",
            |mut caller: Caller<'_, D>, offset: u32, size: u32| {
                info!("{}", Memory::read_string_from_caller(&mut caller, offset, size)?);
                Ok(())
            },
        )?;
        inner.func_wrap(
            "logging",
            "debug",
            |mut caller: Caller<'_, D>, offset: u32, size: u32| {
                debug!("{}", Memory::read_string_from_caller(&mut caller, offset, size)?);
                Ok(())
            },
        )?;
        inner.func_wrap(
            "logging",
            "trace",
            |mut caller: Caller<'_, D>, offset: u32, size: u32| {
                trace!("{}", Memory::read_string_from_caller(&mut caller, offset, size)?);
                Ok(())
            },
        )?;
        Ok(Self(inner))
    }
}

impl<S: Send> Linker<S> {
    /// Instantiate the module.
    #[instrument(skip_all)]
    pub async fn instantiate_async(
        &self,
        mut store: impl AsContextMut<Data = S>,
        module: &Module,
    ) -> Result<Instance> {
        self.0
            .instantiate_async(store.as_context_mut(), module.as_ref())
            .await
            .with_context(|| format!("failed to instantiate module `{:?}`", module.as_ref().name()))
            .map(Instance::from)
    }
}

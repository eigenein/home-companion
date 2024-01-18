use wasmtime::AsContextMut;

use crate::{
    prelude::{instrument, *},
    wasm::{instance::Instance, module::Module},
};

pub struct Linker<T = ()>(wasmtime::Linker<T>);

impl<T> From<wasmtime::Linker<T>> for Linker<T> {
    fn from(inner: wasmtime::Linker<T>) -> Self {
        Self(inner)
    }
}

impl<S: Send> Linker<S> {
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

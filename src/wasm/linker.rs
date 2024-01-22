use wasmtime::AsContextMut;

use crate::{
    prelude::{instrument, *},
    wasm::{instance::Instance, module::Module},
};

#[derive(derive_more::From, derive_more::AsMut, derive_more::AsRef)]
pub struct Linker<T = ()>(wasmtime::Linker<T>);

impl<D: Send> Linker<D> {
    /// Instantiate the module.
    #[instrument(skip_all)]
    pub async fn instantiate_async(
        &self,
        mut store: impl AsContextMut<Data = D>,
        module: &Module,
    ) -> Result<Instance> {
        self.0
            .instantiate_async(store.as_context_mut(), module.as_ref())
            .await
            .with_context(|| format!("failed to instantiate module `{:?}`", module.as_ref().name()))
            .map(Instance::from)
    }
}

use wasmtime::{AsContextMut, TypedFunc, WasmParams, WasmResults};

use crate::prelude::*;

pub struct Instance(wasmtime::Instance);

impl From<wasmtime::Instance> for Instance {
    fn from(inner: wasmtime::Instance) -> Self {
        Self(inner)
    }
}

impl Instance {
    #[allow(clippy::future_not_send)]
    async fn call_typed_func_async<S: Send, P: WasmParams, R: WasmResults>(
        &self,
        mut store: impl AsContextMut<Data = S>,
        name: &str,
        params: P,
    ) -> Result<R> {
        let function: TypedFunc<P, R> =
            self.0.get_typed_func(&mut store, name).with_context(|| {
                format!(
                    "failed to get function `{name}` from module `{:?}`",
                    self.0.module(&mut store).name(),
                )
            })?;
        function.call_async(&mut store, params).await.with_context(|| {
            format!(
                "failed to call function `{name}` from module `{:?}`",
                self.0.module(&mut store).name(),
            )
        })
    }
}

/// Companion's service connection via WASM module instance.
pub struct Connection(Instance);

impl From<Instance> for Connection {
    fn from(instance: Instance) -> Self {
        Self(instance)
    }
}

impl Connection {
    /// Call the module's `init()`.
    #[allow(clippy::future_not_send)]
    pub async fn call_init_async<S: Send>(&self, store: impl AsContextMut<Data = S>) -> Result {
        self.0.call_typed_func_async(store, "init", ()).await
    }
}

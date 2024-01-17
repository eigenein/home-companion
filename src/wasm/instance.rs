use toml::Table;
use wasmtime::{AsContextMut, TypedFunc, WasmParams, WasmResults};

use crate::{prelude::*, wasm::memory::Memory};

pub struct Instance(wasmtime::Instance);

impl From<wasmtime::Instance> for Instance {
    fn from(inner: wasmtime::Instance) -> Self {
        Self(inner)
    }
}

impl Instance {
    /// # Returns
    ///
    /// String buffer offset and length.
    #[allow(clippy::future_not_send)]
    pub async fn write_string<D: Send>(
        &mut self,
        store: impl AsContextMut<Data = D>,
        value: &str,
    ) -> Result<(usize, usize)> {
        let buffer = value.as_bytes();
        let offset = self.write_bytes(store, buffer).await?;
        Ok((offset, buffer.len()))
    }

    /// # Returns
    ///
    /// Buffer offset.
    #[allow(clippy::future_not_send)]
    pub async fn write_bytes<D: Send>(
        &mut self,
        mut store: impl AsContextMut<Data = D>,
        value: &[u8],
    ) -> Result<usize> {
        let offset =
            self.alloc(&mut store, value.len()).await.context("failed to allocate memory")?;
        self.get_memory(store)?.write_bytes(offset, value)?;
        Ok(offset)
    }

    /// Get the instance's memory named `memory`.
    fn get_memory<S, D>(&self, mut store: S) -> Result<Memory<S>>
    where
        S: AsContextMut<Data = D>,
        D: Send,
    {
        let inner = self.0.get_memory(&mut store, "memory").ok_or_else(|| {
            anyhow!("the module `{:?}` does not export `memory`", self.0.module(&store).name())
        })?;
        Ok(Memory::new(inner, store))
    }

    /// Allocate a buffer of `size` bytes in the instance's memory.
    ///
    /// # Returns
    ///
    /// Offset of the allocated buffer.
    #[allow(clippy::future_not_send)]
    async fn alloc<S: Send>(
        &self,
        store: impl AsContextMut<Data = S>,
        size: usize,
    ) -> Result<usize> {
        let offset: u32 = self.call_typed_func_async(store, "alloc", u32::try_from(size)?).await?;
        Ok(usize::try_from(offset)?)
    }

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
    /// Call the module's `init(i32, i32) -> (i32, i32)`.
    ///
    /// The `init()` must accept a UTF-8 encoded TOML configuration string (address and length),
    /// and return a state byte string (address and length). Companion will allocate and write
    /// the string to the memory prior to calling the `init()`.
    ///
    /// Return length may be equal to `0` – in that case, Companion does not need access to the memory.
    #[allow(clippy::future_not_send)]
    pub async fn call_init_async<D: Send>(
        &mut self,
        mut store: impl AsContextMut<Data = D>,
        settings: &Table,
    ) -> Result {
        let settings = toml::to_string(settings)?;
        let (offset, size) = self.0.write_string(&mut store, &settings).await?;
        self.0
            .call_typed_func_async(store, "init", (u32::try_from(offset)?, u32::try_from(size)?))
            .await
    }
}

use toml::Table;
use wasmtime::{AsContextMut, TypedFunc, WasmParams, WasmResults};

use crate::{
    helpers::serde::transcode_toml_to_message_pack,
    prelude::*,
    wasm::memory::{Memory, Segment},
};

pub struct Instance(pub wasmtime::Instance);

impl Instance {
    #[deprecated]
    #[allow(clippy::future_not_send)]
    async fn call_typed_func_async<S: Send, P: WasmParams, R: WasmResults>(
        &self,
        mut store: impl AsContextMut<Data = S>,
        name: &str,
        params: P,
    ) -> Result<R> {
        let function: TypedFunc<P, R> = self
            .0
            .get_typed_func(store.as_context_mut(), name)
            .with_context(|| format!("failed to get function `{name}`",))?;
        function
            .call_async(store.as_context_mut(), params)
            .await
            .with_context(|| format!("failed to call function `{name}` from module",))
    }
}

/// Companion's service connection via WASM module instance.
pub struct Connection(pub Instance);

impl Connection {
    /// Call the module's `init(i32, i32) -> (i32, i32)`.
    ///
    /// The `init()` must accept a MessagePack-encoded byte string (address and length),
    /// and return a Protocol Buffers-encoded state byte string. Companion will allocate and write
    /// the string to the memory prior to calling the `init()`.
    ///
    /// Return length may be equal to `0` – in that case, Companion does not need access to the memory.
    ///
    /// # Returns
    ///
    /// Byte string, returned by the `init()`.
    #[allow(clippy::future_not_send)]
    pub async fn call_init_async<D: Send>(
        &mut self,
        mut store: impl AsContextMut<Data = D>,
        settings: Table,
    ) -> Result<Vec<u8>> {
        let settings = transcode_toml_to_message_pack(settings)?;
        let memory = Memory::try_from_instance(store.as_context_mut(), &self.0.0)?;
        let segment = memory.write_bytes(store.as_context_mut(), &settings).await?;
        let (offset, size): (u32, u32) = self
            .0
            .call_typed_func_async(store.as_context_mut(), "init", segment.as_tuple_u32()?)
            .await?;
        memory.read_bytes(store.as_context_mut(), Segment::try_from_u32(offset, size)?)
    }
}

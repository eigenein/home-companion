use toml::Table;
use wasmtime::AsContextMut;

use crate::{
    helpers::serde::transcode_toml_to_message_pack,
    prelude::*,
    wasm::{r#extern::Extern, function::InitFunction, memory::Memory},
};

#[derive(derive_more::From, derive_more::AsRef)]
pub struct Instance(wasmtime::Instance);

/// Companion's service connection via WASM module instance.
#[derive(derive_more::From, derive_more::AsRef)]
pub struct Connection(Instance);

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
    pub async fn call_init_async<D: Send>(
        &mut self,
        mut store: impl AsContextMut<Data = D>,
        settings: Table,
    ) -> Result<Vec<u8>> {
        let settings = transcode_toml_to_message_pack(settings)?;
        let memory = Memory::try_from_instance(store.as_context_mut(), &self.0.0)?;
        let settings_segment = memory.write_bytes(store.as_context_mut(), &settings).await?;
        let init_func = InitFunction::try_from_instance(store.as_context_mut(), &self.0.0)?;
        let state_segment = init_func.call_async(store.as_context_mut(), settings_segment).await?;
        memory.read_bytes(store.as_context_mut(), state_segment)
    }
}

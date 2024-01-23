use toml::Table;
use wasmtime::AsContextMut;

use crate::{
    helpers::serde::transcode_toml_to_message_pack,
    wasm::{
        r#extern::TryFromInstance, function::InitGuestFunction, instance::Instance, memory::Memory,
    },
};

/// Companion's service connection via WASM module instance.
#[derive(derive_more::From, derive_more::AsRef)]
#[must_use]
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
    pub async fn init_async<D: Send>(
        &mut self,
        mut store: impl AsContextMut<Data = D>,
        settings: Table,
    ) -> crate::prelude::Result<Vec<u8>> {
        let settings = transcode_toml_to_message_pack(settings)?;
        let memory = Memory::try_from_instance(store.as_context_mut(), self.0.as_ref())?;
        InitGuestFunction::try_from_instance(store.as_context_mut(), self.0.as_ref())?
            .call_async(store.as_context_mut(), &memory, &settings)
            .await
    }
}

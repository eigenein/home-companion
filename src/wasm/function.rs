use home_companion_sdk::memory::BufferDescriptor;
use wasmtime::{AsContext, AsContextMut, Extern, WasmParams, WasmResults};

use crate::{
    prelude::*,
    wasm::{
        r#extern::{ExternName, TryFromExtern, WrapExtern},
        memory::Memory,
    },
};

/// Generic typed guest function wrapper.
#[must_use]
pub struct TypedGuestFunction<Params, Results>(wasmtime::TypedFunc<Params, Results>);

impl<Params: WasmParams, Results: WasmResults> TryFromExtern
    for TypedGuestFunction<Params, Results>
{
    fn try_from_extern(store: impl AsContext, extern_: Extern) -> Result<Self> {
        extern_
            .into_func()
            .ok_or_else(|| anyhow!("the export is not a function"))?
            .typed(store)
            .context("failed to extract a typed function")
            .map(Self)
    }
}

/// Allocate memory block of the specified size and return the block offset.
#[derive(derive_more::From)]
#[must_use]
pub struct AllocFunction(TypedGuestFunction<(u32,), u32>);

impl ExternName for AllocFunction {
    const NAME: &'static str = "alloc";
}

impl WrapExtern for AllocFunction {
    type Inner = TypedGuestFunction<(u32,), u32>;
}

impl AllocFunction {
    pub async fn call_async<D: Send>(
        &self,
        store: impl AsContextMut<Data = D>,
        size: u32,
    ) -> Result<u32> {
        self.0.0.call_async(store, (size,)).await.context("failed to allocate memory")
    }
}

/// Call the module's `init(i64) -> i64`.
///
/// The `init()` must accept a MessagePack-encoded settings, and return a state byte string.
#[derive(derive_more::From)]
#[must_use]
pub struct InitGuestFunction(TypedGuestFunction<(u64,), u64>);

impl ExternName for InitGuestFunction {
    const NAME: &'static str = "init";
}

impl WrapExtern for InitGuestFunction {
    type Inner = TypedGuestFunction<(u64,), u64>;
}

impl InitGuestFunction {
    /// Initialize the guest. Accepts MessagePack-encoded settings and returns a binary state.
    pub async fn call_async<D: Send>(
        &self,
        mut store: impl AsContextMut<Data = D>,
        memory: &Memory,
        settings: &[u8],
    ) -> Result<Vec<u8>> {
        let descriptor = memory.write_bytes(store.as_context_mut(), settings).await?;
        let descriptor = self
            .0
            .0
            .call_async(store.as_context_mut(), (descriptor.into(),))
            .await
            .context("failed to call `init()`")?;
        memory.read_bytes(store, BufferDescriptor::from_raw(descriptor))
    }
}

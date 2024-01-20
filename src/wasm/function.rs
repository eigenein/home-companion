use wasmtime::{AsContext, AsContextMut, Extern, WasmParams, WasmResults};

use crate::{
    prelude::*,
    wasm::{
        r#extern::{ExternName, TryFromExtern, WrapExtern},
        memory::{Memory, Segment},
    },
};

/// Generic typed guest function wrapper.
pub struct TypedGuestFunction<Params, Results>(wasmtime::TypedFunc<Params, Results>);

impl<Params: WasmParams, Results: WasmResults> TryFromExtern
    for TypedGuestFunction<Params, Results>
{
    fn try_from_extern(store: impl AsContext, extern_: Extern) -> Result<Self> {
        extern_
            .into_func()
            .ok_or_else(|| anyhow!("the export is not a function"))?
            .typed(store.as_context())
            .context("failed to extract a typed function")
            .map(Self)
    }
}

/// Allocate memory block of the specified size and return the block offset.
#[derive(derive_more::From)]
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
        mut store: impl AsContextMut<Data = D>,
        size: usize,
    ) -> Result<usize> {
        Ok(self
            .0
            .0
            .call_async(store.as_context_mut(), (u32::try_from(size)?,))
            .await
            .context("failed to allocate memory")?
            .try_into()?)
    }
}

/// Call the module's `init(i32, i32) -> (i32, i32)`.
///
/// The `init()` must accept a MessagePack-encoded settings, and return a state byte string.
///
/// Return length may be equal to `0` – in that case, Companion does not need access to the memory.
#[derive(derive_more::From)]
pub struct InitGuestFunction(TypedGuestFunction<(u32, u32), (u32, u32)>);

impl ExternName for InitGuestFunction {
    const NAME: &'static str = "init";
}

impl WrapExtern for InitGuestFunction {
    type Inner = TypedGuestFunction<(u32, u32), (u32, u32)>;
}

impl InitGuestFunction {
    /// Initialize the guest. Accepts MessagePack-encoded settings and returns a binary state.
    pub async fn call_async<D: Send>(
        &self,
        mut store: impl AsContextMut<Data = D>,
        memory: &Memory,
        settings: &[u8],
    ) -> Result<Vec<u8>> {
        let params = memory.write_bytes(store.as_context_mut(), settings).await?.as_tuple_u32()?;
        let (state_offset, size_offset) = self
            .0
            .0
            .call_async(store.as_context_mut(), params)
            .await
            .context("failed to call `init()`")?;
        memory.read_bytes(store.as_context_mut(), Segment::try_from_u32(state_offset, size_offset)?)
    }
}

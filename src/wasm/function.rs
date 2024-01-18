use wasmtime::{AsContext, AsContextMut, Extern, WasmParams, WasmResults};

use crate::{prelude::*, wasm::memory::Segment};

pub struct TypedFunction<Params, Results>(wasmtime::TypedFunc<Params, Results>);

impl<Params: WasmParams, Results: WasmResults> TypedFunction<Params, Results> {
    pub fn try_from_extern<D>(
        store: impl AsContext<Data = D>,
        extern_: Option<Extern>,
    ) -> Result<Self> {
        extern_
            .ok_or_else(|| anyhow!("function is not exported"))?
            .into_func()
            .ok_or_else(|| anyhow!("the export is not a function"))?
            .typed(store.as_context())
            .context("failed to extract a typed function")
            .map(Self)
    }

    pub fn try_from_instance<D>(
        mut store: impl AsContextMut<Data = D>,
        instance: &wasmtime::Instance,
        name: &str,
    ) -> Result<Self> {
        let extern_ = instance.get_export(store.as_context_mut(), name);
        Self::try_from_extern(store.as_context(), extern_)
            .with_context(|| format!("failed to extract a typed function `{name}`"))
    }
}

pub trait TryFromInstance<Params, Results>: From<TypedFunction<Params, Results>>
where
    Params: WasmParams,
    Results: WasmResults,
{
    const NAME: &'static str;

    fn try_from_instance<D>(
        mut store: impl AsContextMut<Data = D>,
        instance: &wasmtime::Instance,
    ) -> Result<Self> {
        TypedFunction::try_from_instance(store.as_context_mut(), instance, Self::NAME)
            .map(Self::from)
    }
}

/// Allocate memory.
///
/// # Params
///
/// Number of bytes to allocate.
///
/// # Returns
///
/// Offset of an allocated block.
#[derive(derive_more::From)]
pub struct AllocFunction(TypedFunction<(u32,), u32>);

impl TryFromInstance<(u32,), u32> for AllocFunction {
    const NAME: &'static str = "alloc";
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
/// The `init()` must accept a MessagePack-encoded byte string (address and length),
/// and return a Protocol Buffers-encoded state byte string. Companion will allocate and write
/// the string to the memory prior to calling the `init()`.
///
/// Return length may be equal to `0` – in that case, Companion does not need access to the memory.
///
/// # Returns
///
/// Byte string, returned by the `init()`.
#[derive(derive_more::From)]
pub struct InitFunction(TypedFunction<(u32, u32), (u32, u32)>);

impl TryFromInstance<(u32, u32), (u32, u32)> for InitFunction {
    const NAME: &'static str = "init";
}

impl InitFunction {
    pub async fn call_async<D: Send>(
        &self,
        mut store: impl AsContextMut<Data = D>,
        settings_segment: Segment,
    ) -> Result<Segment> {
        let (state_offset, size_offset) = self
            .0
            .0
            .call_async(store.as_context_mut(), settings_segment.as_tuple_u32()?)
            .await
            .context("failed to call `init()`")?;
        Segment::try_from_u32(state_offset, size_offset)
    }
}

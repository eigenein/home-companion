use std::ops::Deref;

use wasmtime::{AsContext, Extern, WasmParams, WasmResults};

use crate::prelude::*;

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
}

impl<Params, Results> Deref for TypedFunction<Params, Results> {
    type Target = wasmtime::TypedFunc<Params, Results>;

    fn deref(&self) -> &Self::Target {
        &self.0
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
pub type AllocFunction = TypedFunction<(u32,), u32>;

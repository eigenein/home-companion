use wasmtime::{AsContextMut, Caller, Instance};

use crate::prelude::*;

pub trait TryFromExtern: Sized {
    /// Instantiate current type from the native WASM extern.
    fn try_from_extern(store: impl AsContextMut, extern_: wasmtime::Extern) -> Result<Self>;
}

pub trait WrapExtern {
    /// Wrapped extern type.
    type Inner: TryFromExtern;
}

/// Implement [`TryFromExtern`] for extern wrappers.
impl<T: WrapExtern + From<T::Inner>> TryFromExtern for T {
    fn try_from_extern(store: impl AsContextMut, extern_: wasmtime::Extern) -> Result<Self> {
        T::Inner::try_from_extern(store, extern_).map(T::from)
    }
}

pub trait ExternName {
    /// Native WASM extern name.
    const NAME: &'static str;
}

pub trait TryFromInstance: Sized {
    /// Retrieve an extern from the module instance.
    fn try_from_instance(store: impl AsContextMut, instance: &Instance) -> Result<Self>;
}

impl<T: ExternName + TryFromExtern> TryFromInstance for T {
    fn try_from_instance(
        mut store: impl AsContextMut,
        instance: &wasmtime::Instance,
    ) -> Result<Self> {
        let extern_ = instance
            .get_export(store.as_context_mut(), Self::NAME)
            .ok_or_else(|| anyhow!("failed to look up instance's extern `{}`", Self::NAME))?;
        Self::try_from_extern(store.as_context_mut(), extern_)
    }
}

pub trait TryFromCaller<D>: Sized {
    /// Retrieve an extern from the caller.
    fn try_from_caller(caller: &mut Caller<D>) -> Result<Self>;
}

impl<T: ExternName + TryFromExtern, D> TryFromCaller<D> for T {
    fn try_from_caller(caller: &mut Caller<D>) -> Result<Self> {
        let extern_ = caller
            .get_export(Self::NAME)
            .ok_or_else(|| anyhow!("failed to look up caller's extern `{}`", Self::NAME))?;
        Self::try_from_extern(caller.as_context_mut(), extern_)
    }
}

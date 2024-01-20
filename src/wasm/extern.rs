use wasmtime::{AsContextMut, Caller};

use crate::prelude::*;

pub trait TryFromExtern: Sized {
    /// Convert the extern into the current type.
    fn try_from_extern(store: impl AsContextMut, extern_: wasmtime::Extern) -> Result<Self>;
}

pub trait ExternDeclaration: Sized {
    /// Type to convert native WASM extern into.
    type Inner: TryFromExtern + Into<Self>;

    /// Exported name.
    const NAME: &'static str;
}

pub trait TryFromInstance: Sized {
    /// Retrieve an extern from the module instance.
    fn try_from_instance(store: impl AsContextMut, instance: &wasmtime::Instance) -> Result<Self>;
}

pub trait TryFromCaller<D>: Sized {
    /// Retrieve an extern from the caller.
    fn try_from_caller(caller: &mut Caller<D>) -> Result<Self>;
}

impl<T: ExternDeclaration> TryFromInstance for T {
    fn try_from_instance(
        mut store: impl AsContextMut,
        instance: &wasmtime::Instance,
    ) -> Result<Self> {
        let extern_ = instance
            .get_export(store.as_context_mut(), Self::NAME)
            .ok_or_else(|| anyhow!("failed to look up extern `{}`", Self::NAME))?;
        Self::try_from_extern(store.as_context_mut(), extern_)
    }
}

impl<T: ExternDeclaration, D> TryFromCaller<D> for T {
    fn try_from_caller(caller: &mut Caller<D>) -> Result<Self> {
        let extern_ = caller
            .get_export(Self::NAME)
            .ok_or_else(|| anyhow!("failed to look up extern `{}`", Self::NAME))?;
        Self::try_from_extern(caller.as_context_mut(), extern_)
    }
}

impl<T: ExternDeclaration> TryFromExtern for T {
    fn try_from_extern(mut store: impl AsContextMut, extern_: wasmtime::Extern) -> Result<Self> {
        Ok(T::Inner::try_from_extern(store.as_context_mut(), extern_)?.into())
    }
}

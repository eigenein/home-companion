use wasmtime::{AsContextMut, Caller};

use crate::prelude::*;

pub trait TryFromExtern: Sized {
    /// Convert the extern into the current type.
    fn try_from_extern(store: impl AsContextMut, extern_: wasmtime::Extern) -> Result<Self>;
}

pub trait ExternDeclaration {
    /// Type to convert native WASM extern into.
    type Inner;

    /// Exported name.
    const NAME: &'static str;
}

pub trait Extern<D>: Sized {
    fn try_from_instance(
        store: impl AsContextMut<Data = D>,
        instance: &wasmtime::Instance,
    ) -> Result<Self>;

    fn try_from_caller(caller: &mut Caller<D>) -> Result<Self>;

    fn try_from_extern(
        store: impl AsContextMut<Data = D>,
        extern_: wasmtime::Extern,
    ) -> Result<Self>;
}

impl<T, D> Extern<D> for T
where
    T: ExternDeclaration + From<T::Inner>,
    <T as ExternDeclaration>::Inner: TryFromExtern,
{
    fn try_from_instance(
        mut store: impl AsContextMut<Data = D>,
        instance: &wasmtime::Instance,
    ) -> Result<Self> {
        let extern_ = instance
            .get_export(store.as_context_mut(), Self::NAME)
            .ok_or_else(|| anyhow!("failed to look up extern `{}`", Self::NAME))?;
        Self::try_from_extern(store.as_context_mut(), extern_)
    }

    fn try_from_caller(caller: &mut Caller<D>) -> Result<Self> {
        let extern_ = caller
            .get_export(Self::NAME)
            .ok_or_else(|| anyhow!("failed to look up extern `{}`", Self::NAME))?;
        Self::try_from_extern(caller.as_context_mut(), extern_)
    }

    fn try_from_extern(
        mut store: impl AsContextMut<Data = D>,
        extern_: wasmtime::Extern,
    ) -> Result<Self> {
        Ok(T::Inner::try_from_extern(store.as_context_mut(), extern_)?.into())
    }
}

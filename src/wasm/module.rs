use std::ops::Deref;

/// WASM module wrapper.
pub struct Module(wasmtime::Module);

impl From<wasmtime::Module> for Module {
    fn from(inner: wasmtime::Module) -> Self {
        Self(inner)
    }
}

impl Deref for Module {
    type Target = wasmtime::Module;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

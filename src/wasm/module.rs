/// WASM module wrapper.
#[derive(derive_more::From, derive_more::AsRef)]
#[must_use]
pub struct Module(wasmtime::Module);

#[must_use]
pub struct StatefulModule {
    pub module: Module,

    /// Arbitrary serialized module-specific state.
    ///
    /// An initial state is returned from the module's `init()`.
    pub state: Vec<u8>,
}

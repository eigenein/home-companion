/// WASM module wrapper.
#[derive(derive_more::From, derive_more::AsRef)]
pub struct Module(wasmtime::Module);

pub struct StatefulModule {
    pub module: Module,

    /// Arbitrary serialized module-specific state.
    ///
    /// An initial state is returned from the module's `init()`.
    pub state: Vec<u8>,
}

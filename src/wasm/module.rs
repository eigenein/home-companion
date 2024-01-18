/// WASM module wrapper.
#[derive(derive_more::From, derive_more::AsRef)]
pub struct Module(wasmtime::Module);

pub struct StatefulModule {
    pub module: Module,
    pub state: Vec<u8>,
}

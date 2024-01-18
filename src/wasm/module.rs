/// WASM module wrapper.
pub struct Module(pub wasmtime::Module);

pub struct StatefulModule {
    pub module: Module,
    pub state: Vec<u8>,
}

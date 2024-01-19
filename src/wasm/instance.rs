#[derive(derive_more::From, derive_more::AsRef)]
pub struct Instance(wasmtime::Instance);

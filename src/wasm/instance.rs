#[derive(derive_more::From, derive_more::AsRef)]
#[must_use]
pub struct Instance(wasmtime::Instance);

use wasmtime::AsContextMut;

use crate::prelude::*;

pub struct Memory<S> {
    inner: wasmtime::Memory,
    store: S,
}

impl<S> Memory<S> {
    pub const fn new(inner: wasmtime::Memory, store: S) -> Self {
        Self { inner, store }
    }
}

impl<S: AsContextMut<Data = D>, D: Send> Memory<S> {
    #[allow(clippy::future_not_send)]
    pub fn read_bytes(&self, offset: usize, size: usize) -> Result<Vec<u8>> {
        let mut buffer = Vec::with_capacity(size);
        self.inner
            .read(&self.store, offset, &mut buffer)
            .context("failed to read the instance's memory")?;
        Ok(buffer)
    }

    #[allow(clippy::future_not_send)]
    pub fn write_bytes(&mut self, offset: usize, value: &[u8]) -> Result {
        self.inner
            .write(&mut self.store, offset, value)
            .context("failed to write the buffer into the instance's memory")
    }
}

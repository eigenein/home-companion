use home_companion_sdk::memory::BufferDescriptor;
use prost::Message;
use wasmtime::{AsContext, AsContextMut, Caller, Instance};

use crate::{
    prelude::*,
    wasm::{
        r#extern::{TryFromCaller, TryFromInstance},
        function::AllocFunction,
    },
};

/// WASM guest memory wrapper.
#[must_use]
pub struct Memory(wasmtime::Memory, AllocFunction);

impl TryFromCaller for Memory {
    fn try_from_caller<D>(caller: &mut Caller<'_, D>) -> Result<Self> {
        let memory_extern = caller.get_export("memory");
        let alloc = AllocFunction::try_from_caller(caller)?;
        Self::new(memory_extern, alloc)
    }
}

impl TryFromInstance for Memory {
    fn try_from_instance(mut store: impl AsContextMut, instance: &Instance) -> Result<Self> {
        let memory_extern = instance.get_export(store.as_context_mut(), "memory");
        let alloc = AllocFunction::try_from_instance(store.as_context_mut(), instance)?;
        Self::new(memory_extern, alloc)
    }
}

impl Memory {
    fn new(memory_extern: Option<wasmtime::Extern>, alloc: AllocFunction) -> Result<Self> {
        let inner = memory_extern
            .ok_or_else(|| anyhow!("module does not export `memory`"))?
            .into_memory()
            .ok_or_else(|| anyhow!("`memory` export is not a memory"))?;
        Ok(Self(inner, alloc))
    }

    pub fn read_bytes(
        &self,
        store: impl AsContext,
        descriptor: BufferDescriptor,
    ) -> Result<Vec<u8>> {
        let (offset, size) = descriptor.split();
        let mut buffer = vec![0; size as usize];
        if size != 0 {
            self.0.read(store.as_context(), offset as usize, &mut buffer).with_context(|| {
                format!("failed to read `{size}` bytes from the memory at offset `{offset}`")
            })?;
        }
        Ok(buffer)
    }

    pub fn read_message<M: Message + Default>(
        &self,
        store: impl AsContext,
        descriptor: BufferDescriptor,
    ) -> Result<M> {
        M::decode(&*self.read_bytes(store, descriptor)?).context("failed to decode a message")
    }

    /// Write the byte string into the instance's memory and return the string buffer offset.
    ///
    /// Byte buffer is automatically allocated in the instance's memory.
    /// The module **must** export `memory` and `alloc()` function.
    pub async fn write_bytes<D: Send>(
        &self,
        mut store: impl AsContextMut<Data = D>,
        value: &[u8],
    ) -> Result<BufferDescriptor> {
        #[allow(clippy::cast_possible_truncation)]
        let size = value.len() as u32;

        let offset = self.1.call_async(store.as_context_mut(), size).await?;
        self.0
            .write(store.as_context_mut(), offset as usize, value)
            .context("failed to write the buffer into the instance's memory")?;
        Ok(BufferDescriptor::new(offset, size))
    }
}

use wasmtime::{AsContext, AsContextMut, Caller};

use crate::{
    prelude::*,
    wasm::{
        r#extern::{TryFromCaller, TryFromInstance},
        function::AllocFunction,
    },
};

pub struct Memory(wasmtime::Memory, AllocFunction);

impl Memory {
    fn new(memory_extern: Option<wasmtime::Extern>, alloc: AllocFunction) -> Result<Self> {
        let inner = memory_extern
            .ok_or_else(|| anyhow!("module does not export `memory`"))?
            .into_memory()
            .ok_or_else(|| anyhow!("`memory` export is not a memory"))?;
        Ok(Self(inner, alloc))
    }

    pub fn try_from_caller<D>(caller: &mut Caller<'_, D>) -> Result<Self> {
        let memory_extern = caller.get_export("memory");
        let alloc = AllocFunction::try_from_caller(caller)?;
        Self::new(memory_extern, alloc)
    }

    pub fn try_from_instance<D>(
        mut store: impl AsContextMut<Data = D>,
        instance: &wasmtime::Instance,
    ) -> Result<Self> {
        let memory_extern = instance.get_export(store.as_context_mut(), "memory");
        let alloc = AllocFunction::try_from_instance(store.as_context_mut(), instance)?;
        Self::new(memory_extern, alloc)
    }

    pub fn read_bytes<D: Send>(
        &self,
        store: impl AsContext<Data = D>,
        segment: Segment,
    ) -> Result<Vec<u8>> {
        let mut buffer = vec![0; segment.size];
        if segment.size != 0 {
            self.0
                .read(store.as_context(), segment.offset, &mut buffer)
                .context("failed to read the instance's memory")?;
        }
        Ok(buffer)
    }

    /// Write the byte string into the instance's memory.
    ///
    /// Byte buffer is automatically allocated in the instance's memory.
    /// The module **must** export `memory` and `alloc()` function.
    ///
    /// # Returns
    ///
    /// Buffer offset.
    pub async fn write_bytes<D: Send>(
        &self,
        mut store: impl AsContextMut<Data = D>,
        value: &[u8],
    ) -> Result<Segment> {
        let offset = self.1.call_async(store.as_context_mut(), value.len()).await?;
        self.0
            .write(store.as_context_mut(), offset, value)
            .context("failed to write the buffer into the instance's memory")?;
        Ok(Segment::new(offset, value.len()))
    }
}

/// Continuous segment of WASM instance's memory.
#[derive(Copy, Clone)]
pub struct Segment {
    pub offset: usize,
    pub size: usize,
}

impl Segment {
    pub const fn new(offset: usize, size: usize) -> Self {
        Self { offset, size }
    }

    pub fn try_from_u32(offset: u32, size: u32) -> Result<Self> {
        Ok(Self {
            offset: usize::try_from(offset)?,
            size: usize::try_from(size)?,
        })
    }

    pub fn as_tuple_u32(self) -> Result<(u32, u32)> {
        Ok((u32::try_from(self.offset)?, u32::try_from(self.size)?))
    }
}

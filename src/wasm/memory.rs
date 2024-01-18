use wasmtime::{AsContext, AsContextMut, Caller, Extern};

use crate::{prelude::*, wasm::function::AllocFunction};

pub struct Memory(wasmtime::Memory, AllocFunction);

impl Memory {
    pub fn try_from_extern<D>(
        store: impl AsContext<Data = D>,
        memory_extern: Option<Extern>,
        alloc_extern: Option<Extern>,
    ) -> Result<Self> {
        let inner = memory_extern
            .ok_or_else(|| anyhow!("module does not export `memory`"))?
            .into_memory()
            .ok_or_else(|| anyhow!("`memory` export is not a memory"))?;
        let alloc = AllocFunction::try_from_extern(store.as_context(), alloc_extern)?;
        Ok(Self(inner, alloc))
    }

    pub fn try_from_caller<D>(caller: &mut Caller<'_, D>) -> Result<Self> {
        let memory_extern = caller.get_export("memory");
        let alloc_extern = caller.get_export("alloc");
        Self::try_from_extern(caller.as_context(), memory_extern, alloc_extern)
    }

    pub fn try_from_instance<D>(
        mut store: impl AsContextMut<Data = D>,
        instance: &wasmtime::Instance,
    ) -> Result<Self> {
        let memory_extern = instance.get_export(store.as_context_mut(), "memory");
        let alloc_extern = instance.get_export(store.as_context_mut(), "alloc");
        Self::try_from_extern(store.as_context(), memory_extern, alloc_extern)
    }

    #[allow(clippy::future_not_send)]
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
    #[allow(clippy::future_not_send)]
    pub async fn write_bytes<D: Send>(
        &self,
        mut store: impl AsContextMut<Data = D>,
        value: &[u8],
    ) -> Result<Segment> {
        let offset = self.alloc(store.as_context_mut(), value.len()).await?;
        self.0
            .write(store.as_context_mut(), offset, value)
            .context("failed to write the buffer into the instance's memory")?;
        Ok(Segment::new(offset, value.len()))
    }

    /// Allocate a buffer of `size` bytes in the instance's memory.
    ///
    /// # Returns
    ///
    /// Offset of the allocated buffer.
    #[allow(clippy::future_not_send)]
    async fn alloc<S: Send>(
        &self,
        mut store: impl AsContextMut<Data = S>,
        size: usize,
    ) -> Result<usize> {
        Ok(self
            .1
            .call_async(store.as_context_mut(), (size.try_into()?,))
            .await
            .context("failed to allocate memory")?
            .try_into()?)
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

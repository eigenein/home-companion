use wasmtime::Extern;

use crate::prelude::*;

pub struct Memory(wasmtime::Memory); // TODO: might store `alloc` function as well.

impl Memory {
    /// Instantiate a memory instance from a `get_export()` closure.
    pub fn from_export(get_export: impl FnOnce(&str) -> Option<Extern>) -> Result<Self> {
        get_export("memory")
            .ok_or_else(|| anyhow!("module must export `memory`"))?
            .into_memory()
            .ok_or_else(|| anyhow!("`memory` export is not a memory"))
            .map(Self)
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

    pub fn from_u32(offset: u32, size: u32) -> Result<Self> {
        Ok(Self {
            offset: usize::try_from(offset)?,
            size: usize::try_from(size)?,
        })
    }

    pub fn as_tuple_u32(self) -> Result<(u32, u32)> {
        Ok((u32::try_from(self.offset)?, u32::try_from(self.size)?))
    }
}

use crate::prelude::*;

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

    pub fn as_offset_u32(self) -> Result<u32> {
        Ok(u32::try_from(self.offset)?)
    }

    pub fn as_size_u32(self) -> Result<u32> {
        Ok(u32::try_from(self.size)?)
    }

    pub fn as_tuple_u32(self) -> Result<(u32, u32)> {
        Ok((self.as_offset_u32()?, self.as_size_u32()?))
    }
}

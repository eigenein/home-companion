use std::{num::TryFromIntError, slice::from_raw_parts_mut};

/// Allocate memory with the global allocator.
///
/// Convenience shortcut for Companion plugins.
///
/// # Panics
///
/// Bad memory layout.
#[must_use]
pub fn alloc(size: usize) -> *mut u8 {
    let layout = std::alloc::Layout::array::<u8>(size).expect("bad memory layout");
    unsafe { std::alloc::alloc(layout) }
}

/// Packed reference to a memory slice (offset and size).
#[derive(Copy, Clone, derive_more::From, derive_more::Into)]
#[must_use]
#[repr(transparent)]
pub struct Segment(u64);

impl Segment {
    /// Pack offset and size into a single 128-bit slice reference.
    pub fn new(offset: usize, size: usize) -> Result<Self, TryFromIntError> {
        Ok(Self(u64::from(u32::try_from(offset)?) | u64::from(u32::try_from(size)?) << 32))
    }

    /// Split the packed slice reference into separate offset and size.
    #[allow(clippy::cast_possible_truncation)]
    pub fn split(self) -> Result<(usize, usize), TryFromIntError> {
        let (offset, size) = (self.0 as u32, (self.0 >> 32) as u32);
        Ok((offset.try_into()?, size.try_into()?))
    }
}

impl TryFrom<&[u8]> for Segment {
    type Error = TryFromIntError;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        Self::new(buffer.as_ptr() as usize, buffer.len())
    }
}

impl TryFrom<Segment> for &[u8] {
    type Error = TryFromIntError;

    fn try_from(segment: Segment) -> Result<Self, Self::Error> {
        let (offset, size) = segment.split()?;
        let pointer = offset as *mut u8;
        Ok(unsafe { from_raw_parts_mut(pointer, size) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_unpack_offset_size() {
        let segment = Segment::new(100500, 42).unwrap();
        assert_eq!(segment.split().unwrap(), (100500, 42));
    }
}

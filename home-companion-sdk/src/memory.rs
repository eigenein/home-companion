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
    /// Pack offset and size into a single 64-bit slice reference.
    pub const fn new(offset: u32, size: u32) -> Self {
        Self(offset as u64 | (size as u64) << 32)
    }

    /// Split the packed slice reference into separate offset and size.
    #[must_use]
    pub const fn split(self) -> (u32, u32) {
        #[allow(clippy::cast_possible_truncation)]
        let offset = self.0 as u32;

        (offset, (self.0 >> 32) as u32)
    }
}

pub trait AsSegment {
    fn as_segment(&self) -> Segment;
}

impl<T: AsRef<[u8]>> AsSegment for T {
    fn as_segment(&self) -> Segment {
        #[allow(clippy::cast_possible_truncation)]
        let size = self.as_ref().len() as u32;

        Segment::new(self.as_ref().as_ptr() as u32, size)
    }
}

impl TryFrom<Segment> for &[u8] {
    type Error = TryFromIntError;

    fn try_from(segment: Segment) -> Result<Self, Self::Error> {
        let (offset, size) = segment.split();
        let pointer = offset as *mut u8;
        Ok(unsafe { from_raw_parts_mut(pointer, size as usize) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_unpack_offset_size() {
        let segment = Segment::new(100500, 42);
        assert_eq!(segment.split(), (100500, 42));
    }
}

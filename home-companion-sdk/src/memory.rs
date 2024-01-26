//! Memory management.

/// Allocate memory with the global allocator.
///
/// Convenience shortcut for Companion plugins.
///
/// # Panics
///
/// Bad memory layout.
#[cfg(feature = "guest")]
#[must_use]
pub fn alloc(size: usize) -> *mut u8 {
    let layout = std::alloc::Layout::array::<u8>(size).expect("bad memory layout");
    unsafe { std::alloc::alloc(layout) }
}

/// Packed reference to a memory slice on WASM module side.
#[derive(Copy, Clone, derive_more::Into)]
#[must_use]
#[repr(transparent)]
pub struct BufferDescriptor(u64);

impl BufferDescriptor {
    #[cfg(feature = "host")]
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

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

/// Convert the slice into a descriptor.
#[cfg(feature = "guest")]
impl<T: AsRef<[u8]>> From<T> for BufferDescriptor {
    fn from(slice: T) -> Self {
        let slice = slice.as_ref();

        #[allow(clippy::cast_possible_truncation)]
        let size = slice.len() as u32;

        Self::new(slice.as_ptr() as u32, size)
    }
}

/// Dereference the descriptor to access teh referenced memory slice.
#[cfg(feature = "guest")]
impl std::ops::Deref for BufferDescriptor {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        let (offset, size) = self.split();
        let pointer = offset as *mut u8;
        unsafe { std::slice::from_raw_parts_mut(pointer, size as usize) }
    }
}

/// Serialize the result and return a serialized buffer descriptor.
#[cfg(feature = "guest")]
impl<T: prost::Message + Default> From<crate::result::RpcResult<T>> for BufferDescriptor {
    fn from(result: crate::result::RpcResult<T>) -> Self {
        use prost::Message;
        result.encode_to_vec().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_descriptor_pack_unpack() {
        let descriptor = BufferDescriptor::new(100500, 42);
        assert_eq!(descriptor.split(), (100500, 42));
    }
}

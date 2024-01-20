pub mod logging;

/// Allocate memory with the global allocator.
///
/// Convenience shortcut for Companion plugins.
pub fn alloc(size: usize) -> *mut u8 {
    let layout = std::alloc::Layout::array::<u8>(size).expect("bad memory layout");
    unsafe { std::alloc::alloc(layout) }
}

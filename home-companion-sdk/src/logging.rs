#[link(wasm_import_module = "logging")]
extern "C" {
    #[link_name = "info"]
    fn _info(message: &str);
}

/// Log message with informational level.
#[inline]
pub fn info(message: &str) {
    unsafe { _info(message) }
}

use crate::memory::Segment;

#[link(wasm_import_module = "logging")]
extern "C" {
    #[link_name = "info"]
    fn _info(message: Segment);

    #[link_name = "error"]
    fn _error(message: Segment);
}

#[inline]
pub fn info(message: &str) {
    unsafe { _info(message.as_bytes().try_into().unwrap()) }
}

#[inline]
pub fn error(message: &str) {
    unsafe { _error(message.as_bytes().try_into().unwrap()) }
}

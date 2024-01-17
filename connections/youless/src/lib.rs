use serde::Deserialize;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    let layout = std::alloc::Layout::array::<u8>(size).expect("bad memory layout");
    unsafe { std::alloc::alloc(layout) }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn init(settings: &[u8]) -> &[u8] {
    let settings: Settings = rmp_serde::from_slice(&settings).expect("failed to parse settings");
    b""
}

#[derive(Deserialize)]
struct Settings {
    host: String,
}

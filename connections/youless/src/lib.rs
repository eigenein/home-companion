use serde::Deserialize;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    unsafe { std::alloc::alloc(std::alloc::Layout::array::<u8>(size).unwrap()) }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn init(settings: &str) -> &[u8] {
    // let settings: toml::Value = toml::from_str(settings).expect("failed to parse settings");
    b""
}

#[derive(Deserialize)]
struct Settings {
    host: String,
}

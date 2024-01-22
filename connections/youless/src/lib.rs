use home_companion_sdk::logging::info;
use prost::Message;
use serde::Deserialize;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    home_companion_sdk::alloc(size)
}

#[no_mangle]
pub extern "C" fn init(settings: &[u8]) -> &[u8] {
    let settings: Settings = rmp_serde::from_slice(&settings).expect("failed to parse settings");
    info(&format!("initializing, hostname = `{}`…", settings.host));
    b""
}

#[derive(Deserialize, Message)]
struct Settings {
    #[prost(string, tag = "1", required)]
    host: String,
}

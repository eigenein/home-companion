mod models;

use anyhow::{Context, Result};
use home_companion_sdk::{
    error::LoggedUnwrap,
    logging::info,
    memory::{AsSegment, Segment},
};

use crate::models::{Counters, Settings};

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    home_companion_sdk::memory::alloc(size)
}

#[no_mangle]
pub extern "C" fn init(settings: Segment) -> Segment {
    let settings: Settings =
        rmp_serde::from_slice(settings.try_into().unwrap()).expect("failed to parse settings");
    let url = format!("http://{}/e", settings.host);

    info(&format!("checking YouLess at `{url}`…"));
    request_counters(&url)
        .with_context(|| format!("failed to request YouLess at `{url}`"))
        .unwrap_logged();

    b"".as_segment()
}

fn request_counters(url: &str) -> Result<Counters> {
    ureq::get(url)
        .call()
        .context("failed to call the YouLess")?
        .into_json()
        .context("failed to deserialize YouLess response")
}

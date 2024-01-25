mod models;

use anyhow::Context;
use home_companion_sdk::{
    abi::{call, Action, HostCall, Log, LogLevel},
    memory::{AsSegment, Segment},
};

use crate::models::{Counters, Settings};

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    home_companion_sdk::memory::alloc(size)
}

#[no_mangle]
pub extern "C" fn init(settings: Segment) -> Segment {
    init_unsafe(settings).unwrap().as_segment() // FIXME
}

fn init_unsafe(settings: Segment) -> anyhow::Result<&'static [u8]> {
    let settings: Settings =
        rmp_serde::from_slice(&settings).context("failed to parse settings")?;
    let url = format!("http://{}/e", settings.host);

    call(&HostCall {
        action: Some(Action::Log(Log {
            message: format!("checking YouLess at `{url}`…"),
            level: LogLevel::Info as i32,
        })),
    });
    // request_counters(&url).with_context(|| format!("failed to request YouLess at `{url}`"))?;

    Ok(b"")
}

fn request_counters(url: &str) -> anyhow::Result<Counters> {
    ureq::get(url)
        .call()
        .context("failed to call the YouLess")?
        .into_json()
        .context("failed to deserialize YouLess response")
}

mod models;

use anyhow::{Context, Result};
use home_companion_sdk::{
    memory::BufferDescriptor,
    result::RpcResult,
    rpc::{call, logging::Log},
};

use crate::models::{Counters, Settings};

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    home_companion_sdk::memory::alloc(size)
}

#[no_mangle]
pub extern "C" fn init(settings: BufferDescriptor) -> BufferDescriptor {
    fn inner(settings: BufferDescriptor) -> Result<Vec<u8>> {
        let settings: Settings =
            rmp_serde::from_slice(&settings).context("failed to parse settings")?;
        let url = format!("http://{}/e", settings.host);

        call(Log::info(format!("checking YouLess at `{url}`…")));
        // request_counters(&url).with_context(|| format!("failed to request YouLess at `{url}`"))?;

        Ok(Vec::new()) // TODO
    }

    RpcResult::from(inner(settings)).into()
}

fn request_counters(url: &str) -> Result<Counters> {
    ureq::get(url)
        .call()
        .context("failed to call the YouLess")?
        .into_json()
        .context("failed to deserialize YouLess response")
}

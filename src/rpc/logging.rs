use home_companion_sdk::rpc::logging::{Log, LogLevel};

use crate::{prelude::*, wasm::state::InstanceId};

pub fn call(instance_id: &InstanceId, log: Log) -> Result {
    let message = log.message;
    match LogLevel::try_from(log.level)? {
        LogLevel::Error => error!(?instance_id, "{message}"),
        LogLevel::Info => info!(?instance_id, "{message}"),
        LogLevel::Debug => debug!(?instance_id, "{message}"),
        LogLevel::Trace => trace!(?instance_id, "{message}"),
    }
    Ok(())
}

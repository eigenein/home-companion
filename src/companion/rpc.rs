use home_companion_sdk::{
    memory::BufferDescriptor,
    result::RpcResult,
    rpc::{logging::LogLevel, Rpc},
};
use wasmtime::{AsContext, Caller, Linker};

use crate::{
    companion::state::HostInstanceState,
    prelude::*,
    wasm::{r#extern::TryFromCaller, memory::Memory},
};

/// Add the RPC handler to the linker.
pub fn add_to<D: Send>(
    linker: &mut Linker<HostInstanceState<D>>,
) -> Result<&mut Linker<HostInstanceState<D>>> {
    linker
        .func_wrap1_async(
            "companion",
            "call",
            |mut caller: Caller<'_, HostInstanceState<D>>, message_descriptor: u64| {
                Box::new(async move {
                    let memory = Memory::try_from_caller(&mut caller)?;
                    let result = handle_call(caller.as_context(), &memory, message_descriptor);
                    memory.write_message(caller, &RpcResult::from(result)).await.map(u64::from)
                })
            },
        )
        .context("failed to link `companion`.`call`")
}

/// Handle the guest's call.
///
/// # Returns
///
/// Serialized response. Inner message can be put inside an [`RpcResult`] directly
/// and parsed correctly on the guest's side with a specific type.
#[inline]
fn handle_call<D>(
    store: impl AsContext<Data = HostInstanceState<D>>,
    memory: &Memory,
    message_descriptor: u64,
) -> Result<Option<Vec<u8>>> {
    let connection_id = store.as_context().data().id.clone();
    let rpc: Rpc = memory
        .read_message(store, BufferDescriptor::from_raw(message_descriptor))
        .with_context(|| format!("failed to read a call from `{connection_id:?}`"))?;
    let action = rpc.action;

    if let Some(log) = action.log {
        let message = log.message;
        match LogLevel::try_from(log.level)? {
            LogLevel::Error => error!(?connection_id, "{message}"),
            LogLevel::Info => info!(?connection_id, "{message}"),
            LogLevel::Debug => debug!(?connection_id, "{message}"),
            LogLevel::Trace => trace!(?connection_id, "{message}"),
        }
        return Ok(None);
    }

    bail!("there is no action in the call")
}

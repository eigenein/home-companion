mod logging;

use home_companion_sdk::{memory::BufferDescriptor, result::RpcResult, rpc::Rpc};
use wasmtime::{AsContext, Caller, Linker};

use crate::{
    prelude::*,
    wasm::{r#extern::TryFromCaller, memory::Memory, state::GuestState},
};

/// Add the RPC handler to the linker.
pub fn add_to<D: Send>(linker: &mut Linker<GuestState<D>>) -> Result<&mut Linker<GuestState<D>>> {
    linker
        .func_wrap1_async(
            "companion",
            "call",
            |mut caller: Caller<'_, GuestState<D>>, message_descriptor: u64| {
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
    store: impl AsContext<Data = GuestState<D>>,
    memory: &Memory,
    message_descriptor: u64,
) -> Result<Option<Vec<u8>>> {
    let instance_id = store.as_context().data().id.clone();
    let rpc: Rpc = memory
        .read_message(store, BufferDescriptor::from_raw(message_descriptor))
        .with_context(|| format!("failed to read a call from `{instance_id:?}`"))?;
    let action = rpc.action;

    if let Some(log) = action.log {
        logging::call(&instance_id, log)?;
        Ok(None)
    } else {
        bail!("there is no action in the call")
    }
}

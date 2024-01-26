use home_companion_sdk::{abi::HostCall, memory::BufferDescriptor};
use wasmtime::{Caller, Linker};

use crate::{
    companion::state::HostInstanceState,
    prelude::*,
    wasm::{r#extern::TryFromCaller, memory::Memory},
};

pub fn add_to<D: Send>(
    linker: &mut Linker<HostInstanceState<D>>,
) -> Result<&mut Linker<HostInstanceState<D>>> {
    linker
        .func_wrap(
            "companion",
            "call",
            |mut caller: Caller<'_, HostInstanceState<D>>, message_descriptor: u64| {
                let connection_id = caller.data().id.clone();
                let message: HostCall = Memory::try_from_caller(&mut caller)?
                    .read_message(&caller, BufferDescriptor::from_raw(message_descriptor))
                    .with_context(|| format!("failed to read a call from `{connection_id:?}`"))?;
                info!(?connection_id, ?message);
                Ok(())
            },
        )
        .context("failed to link `companion`.`call`")
}

use home_companion_sdk::abi::HostCall;
use prost::Message;
use wasmtime::{Caller, Linker};

use crate::{companion::state::HostInstanceState, prelude::*, wasm::memory::Memory};

pub fn add_to<D: Send>(
    linker: &mut Linker<HostInstanceState<D>>,
) -> Result<&mut Linker<HostInstanceState<D>>> {
    linker
        .func_wrap(
            "companion",
            "call",
            |mut caller: Caller<'_, HostInstanceState<D>>, segment: u64| {
                let connection_id = caller.data().id.clone();
                let message = Memory::read_bytes_from_caller(&mut caller, segment.into())
                    .with_context(|| format!("failed to read bytes from `{connection_id:?}`"))?;
                let message = HostCall::decode(message.as_slice())
                    .with_context(|| format!("failed to decode a call from `{connection_id:?}`"))?;
                info!(?connection_id, ?message);
                Ok(())
            },
        )
        .context("failed to link `companion`.`call`")
}

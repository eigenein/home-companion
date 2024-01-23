use wasmtime::Linker;

use crate::{companion::state::HostInstanceState, prelude::*, wasm::memory::Memory};

type Caller<'c, D> = wasmtime::Caller<'c, HostInstanceState<D>>;

/// Add the logging functions to the WASM linker.
pub fn add_to<D: Send>(linker: &mut Linker<HostInstanceState<D>>) -> Result {
    linker.func_wrap("logging", "error", |mut caller: Caller<'_, D>, segment: u64| {
        error!(connection_id = ?caller.data().id, "{}", Memory::read_string_from_caller(&mut caller, segment.into())?);
        Ok(())
    })?;
    linker.func_wrap("logging", "info", |mut caller: Caller<'_, D>, segment: u64| {
        info!(connection_id = ?caller.data().id, "{}", Memory::read_string_from_caller(&mut caller, segment.into())?);
        Ok(())
    })?;
    linker.func_wrap("logging", "debug", |mut caller: Caller<'_, D>, segment: u64| {
        debug!(connection_id = ?caller.data().id, "{}", Memory::read_string_from_caller(&mut caller, segment.into())?);
        Ok(())
    })?;
    linker.func_wrap("logging", "trace", |mut caller: Caller<'_, D>, segment: u64| {
        trace!(connection_id = ?caller.data().id, "{}", Memory::read_string_from_caller(&mut caller, segment.into())?);
        Ok(())
    })?;
    Ok(())
}

use wasmtime::Linker;

use crate::{
    prelude::*,
    wasm::{memory::Memory, state::HostInstanceState},
};

type Caller<'c, D> = wasmtime::Caller<'c, HostInstanceState<D>>;

/// Add the logging functions to the WASM linker.
pub fn add_to_linker<D: Send>(linker: &mut Linker<HostInstanceState<D>>) -> Result {
    linker.func_wrap("logging", "error", |mut caller: Caller<'_, D>, offset: u32, size: u32| {
        error!(connection_id = ?caller.data().id, "{}", Memory::read_string_from_caller(&mut caller, offset, size)?);
        Ok(())
    })?;
    linker.func_wrap("logging", "info", |mut caller: Caller<'_, D>, offset: u32, size: u32| {
        info!(connection_id = ?caller.data().id, "{}", Memory::read_string_from_caller(&mut caller, offset, size)?);
        Ok(())
    })?;
    linker.func_wrap("logging", "debug", |mut caller: Caller<'_, D>, offset: u32, size: u32| {
        debug!(connection_id = ?caller.data().id, "{}", Memory::read_string_from_caller(&mut caller, offset, size)?);
        Ok(())
    })?;
    linker.func_wrap("logging", "trace", |mut caller: Caller<'_, D>, offset: u32, size: u32| {
        trace!(connection_id = ?caller.data().id, "{}", Memory::read_string_from_caller(&mut caller, offset, size)?);
        Ok(())
    })?;
    Ok(())
}

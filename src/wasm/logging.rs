use wasmtime::Caller;

use crate::{prelude::*, wasm::memory::Memory};

/// Add the logging functions to the WASM linker.
pub fn add_to_linker<D: Send>(linker: &mut wasmtime::Linker<D>) -> Result {
    linker.func_wrap("logging", "error", |mut caller: Caller<'_, D>, offset: u32, size: u32| {
        error!("{}", Memory::read_string_from_caller(&mut caller, offset, size)?);
        Ok(())
    })?;
    linker.func_wrap("logging", "info", |mut caller: Caller<'_, D>, offset: u32, size: u32| {
        info!("{}", Memory::read_string_from_caller(&mut caller, offset, size)?);
        Ok(())
    })?;
    linker.func_wrap("logging", "debug", |mut caller: Caller<'_, D>, offset: u32, size: u32| {
        debug!("{}", Memory::read_string_from_caller(&mut caller, offset, size)?);
        Ok(())
    })?;
    linker.func_wrap("logging", "trace", |mut caller: Caller<'_, D>, offset: u32, size: u32| {
        trace!("{}", Memory::read_string_from_caller(&mut caller, offset, size)?);
        Ok(())
    })?;
    Ok(())
}

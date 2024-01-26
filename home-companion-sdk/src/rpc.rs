pub mod action;
pub mod logging;

use anyhow::Context;
use prost::Message;

use crate::{result::RpcResult, rpc::action::Action};

#[cfg(feature = "guest")]
#[link(wasm_import_module = "companion")]
extern "C" {
    #[link_name = "call"]
    fn _call(
        message_descriptor: crate::memory::BufferDescriptor,
    ) -> crate::memory::BufferDescriptor;
}

#[cfg(feature = "guest")]
#[inline]
pub fn call<M: Message + Default>(call: impl Into<Rpc>) -> anyhow::Result<M> {
    let message_descriptor = call.into().encode_to_vec().into();
    let result_descriptor = unsafe { _call(message_descriptor) };
    RpcResult::<M>::decode(&*result_descriptor)
        .context("failed to decode the RPC result")?
        .into()
}

#[derive(Clone, Message)]
#[must_use]
pub struct Rpc {
    #[prost(message, tag = "1", required)]
    pub action: Action,
}

impl<T: Into<Action>> From<T> for Rpc {
    #[inline]
    fn from(action: T) -> Self {
        Self { action: action.into() }
    }
}

pub mod action;
pub mod logging;

use prost::Message;

use crate::rpc::action::Action;

#[cfg(feature = "guest")]
#[link(wasm_import_module = "companion")]
extern "C" {
    #[link_name = "call"]
    fn _call(message_descriptor: crate::memory::BufferDescriptor);
}

#[cfg(feature = "guest")]
#[inline]
pub fn call(call: impl Into<Rpc>) {
    let message_descriptor = call.into().encode_to_vec().into();
    unsafe { _call(message_descriptor) }
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

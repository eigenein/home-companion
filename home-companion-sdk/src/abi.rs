use prost::Message;

use crate::memory::{AsSegment, Segment};

#[link(wasm_import_module = "companion")]
extern "C" {
    #[link_name = "call"]
    fn _call(message: Segment);
}

#[inline]
pub fn call(message: &HostCall) {
    unsafe { _call(message.encode_to_vec().as_segment()) }
}

#[derive(Clone, prost::Message)]
pub struct HostCall {
    #[prost(oneof = "Action", tags = "1")]
    pub action: Option<Action>,
}

#[derive(Clone, prost::Oneof)]
#[must_use]
pub enum Action {
    /// Log the message.
    #[prost(message, tag = "1")]
    Log(Log),
}

#[derive(Clone, prost::Message)]
pub struct Log {
    #[prost(string, tag = "1", required)]
    pub message: String,

    #[prost(enumeration = "LogLevel", tag = "2")]
    pub level: i32,
}

#[derive(Copy, Clone, Debug, prost::Enumeration)]
#[repr(i32)]
pub enum LogLevel {
    Error = 0,
    Info = 10,
    Debug = 20,
    Trace = 30,
}

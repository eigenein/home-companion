use prost::Message;

use crate::rpc::logging::Log;

#[derive(Clone, Message)]
#[must_use]
pub struct Action {
    /// Log the message.
    #[prost(message, tag = "1", optional)]
    pub log: Option<Log>,
}

impl From<Log> for Action {
    #[inline]
    fn from(log: Log) -> Self {
        Self { log: Some(log) }
    }
}

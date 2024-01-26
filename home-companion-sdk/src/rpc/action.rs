use prost::Oneof;

use crate::rpc::logging::Log;

#[derive(Clone, Oneof)]
#[must_use]
pub enum Action {
    /// Log the message.
    #[prost(message, tag = "1")]
    Log(Log),
}

impl From<Log> for Action {
    #[inline]
    fn from(log: Log) -> Self {
        Self::Log(log)
    }
}

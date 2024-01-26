use anyhow::{anyhow, Result};
use prost::{Message, Oneof};

/// Module extern function call result.
#[derive(Clone, Oneof)]
#[must_use]
pub enum RpcResult<T: Message + Default> {
    #[prost(message, tag = "1")]
    Ok(T),

    #[prost(string, tag = "2")]
    Err(String),
}

impl<T: Message + Default> From<Result<T>> for RpcResult<T> {
    fn from(result: Result<T>) -> Self {
        match result {
            Ok(value) => Self::Ok(value),
            Err(error) => Self::Err(format!("{error:#}")),
        }
    }
}

impl<T: Message + Default> From<RpcResult<T>> for Result<T> {
    fn from(result: RpcResult<T>) -> Self {
        match result {
            RpcResult::Ok(value) => Ok(value),
            RpcResult::Err(message) => Err(anyhow!("{message}")),
        }
    }
}

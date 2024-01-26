use anyhow::{anyhow, Result};
use prost::Message;

#[derive(Clone, Message)]
#[must_use]
pub struct RpcResult<T: Message + Default> {
    #[prost(message, tag = "1", optional)]
    pub value: Option<T>,

    #[prost(string, tag = "2", optional)]
    pub error_message: Option<String>,
}

impl<T: Message + Default> From<Result<T>> for RpcResult<T> {
    fn from(result: Result<T>) -> Self {
        match result {
            Ok(value) => Self {
                value: Some(value),
                error_message: None,
            },
            Err(error) => Self {
                value: None,
                error_message: Some(format!("{error:#}")),
            },
        }
    }
}

impl<T: Message + Default> From<RpcResult<T>> for Result<T> {
    fn from(result: RpcResult<T>) -> Self {
        match (result.value, result.error_message) {
            (Some(value), None) => Ok(value),
            (None, Some(error_message)) => Err(anyhow!("{error_message}")),
            (None, None) => Ok(T::default()),
            (Some(value), Some(error_message)) => Err(anyhow!("{error_message}: {value:?}")),
        }
    }
}

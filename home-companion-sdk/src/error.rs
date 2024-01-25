use anyhow::anyhow;

/// Module extern function call result.
#[derive(Clone, prost::Oneof)]
#[must_use]
pub enum Result<T: prost::Message + Default> {
    #[prost(message, tag = "1")]
    Ok(T),

    #[prost(string, tag = "2")]
    Err(String),
}

impl<T: prost::Message + Default> From<anyhow::Result<T>> for Result<T> {
    fn from(result: anyhow::Result<T>) -> Self {
        match result {
            Ok(value) => Self::Ok(value),
            Err(error) => Self::Err(format!("{error:#}")),
        }
    }
}

impl<T: prost::Message + Default> From<Result<T>> for anyhow::Result<T> {
    fn from(result: Result<T>) -> Self {
        match result {
            Result::Ok(value) => Ok(value),
            Result::Err(message) => Err(anyhow!("{message}")),
        }
    }
}

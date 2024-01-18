pub use anyhow::{anyhow, Context};
pub use tracing::{debug, error, info, instrument, trace, warn};

pub type Result<T = ()> = anyhow::Result<T>;
pub type Error = anyhow::Error;

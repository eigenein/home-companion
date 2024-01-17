//! Home setup models.

use std::{
    collections::HashMap,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::prelude::*;

impl Setup {
    pub fn from_file(path: &Path) -> Result<Self> {
        info!(?path, "reading home setup…");
        let content = read_to_string(path)
            .with_context(|| format!("failed to read home setup from `{path:?}`"))?;
        toml::from_str(&content)
            .with_context(|| format!("failed to parse home setup from `{path:?}`"))
    }
}

/// Root home setup.
#[derive(Debug, Deserialize)]
pub struct Setup {
    /// Companion connections to other services.
    ///
    /// Keys are unique IDs, which are used to route messages between connections.
    #[serde(default)]
    pub connections: HashMap<String, Connection>,
}

/// System connection.
#[derive(Debug, Deserialize)]
pub struct Connection {
    /// WASM module path.
    #[serde(alias = "module", alias = "path")]
    pub module_path: PathBuf,

    /// Captures connection-specific settings.
    #[serde(flatten)]
    pub settings: toml::Table,
}

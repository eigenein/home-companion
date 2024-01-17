use std::collections::HashMap;

use futures::{stream, StreamExt, TryStreamExt};

use crate::{
    prelude::*,
    setup::Setup,
    wasm::{engine::Engine, instance::Connection},
};

/// 🚀 The Companion engine.
pub struct Companion {
    connections: HashMap<String, Connection>,
}

impl Companion {
    #[instrument(skip_all)]
    pub async fn from_setup(setup: &Setup) -> Result<Self> {
        info!("loading connections…");

        let engine = Engine::new_async()?;
        let linker = engine.new_linker();

        let connections: HashMap<String, Connection> = {
            let engine = &engine;
            let linker = &linker;
            stream::iter(setup.connections.iter())
                .then(|(id, connection)| async move {
                    info!(id, path = ?connection.module_path, "loading connection…");
                    Ok::<_, Error>((id.to_string(), engine.load_module(&connection.module_path)?))
                })
                .and_then(|(id, module)| async move {
                    let mut store = engine.new_store(());
                    let instance = linker.instantiate_async(&mut store, &module).await?;
                    let connection = Connection::from(instance);
                    connection.call_init_async(&mut store).await?;
                    Ok((id, connection))
                })
                .try_collect()
                .await?
        };

        info!(n_connections = connections.len(), "completed");
        Ok(Self { connections })
    }
}

use std::collections::HashMap;

use futures::{stream, StreamExt, TryStreamExt};
use wasmtime::AsContextMut;

use crate::{
    prelude::*,
    setup::Setup,
    wasm::{engine::Engine, instance::Connection, module::Stateful},
};

/// 🚀 The Companion engine.
pub struct Companion {
    connections: HashMap<String, Stateful>,
}

impl Companion {
    #[instrument(skip_all)]
    pub async fn from_setup(setup: Setup) -> Result<Self> {
        info!("loading connections…");

        let engine = Engine::new_async()?;
        let linker = engine.new_linker()?;

        let connections: HashMap<String, Stateful> = {
            let engine = &engine;
            let linker = &linker;
            stream::iter(setup.connections.into_iter())
                .then(|(id, connection)| async move {
                    info!(id, path = ?connection.module_path, "loading connection…");
                    Ok::<_, Error>((
                        id.to_string(),
                        engine.load_module(&connection.module_path)?,
                        connection.settings,
                    ))
                })
                .and_then(|(id, module, settings)| async move {
                    let mut store = engine.new_store(());
                    let instance =
                        linker.instantiate_async(store.as_context_mut(), &module).await?;
                    let mut connection = Connection::from(instance);
                    let state =
                        connection.call_init_async(store.as_context_mut(), settings).await?;
                    let stateful_module = Stateful { module, state };
                    Ok((id, stateful_module))
                })
                .try_collect()
                .await?
        };

        info!(n_connections = connections.len(), "completed");
        Ok(Self { connections })
    }
}

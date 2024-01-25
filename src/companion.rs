use std::collections::HashMap;

use futures::{stream, StreamExt, TryStreamExt};
use wasmtime::AsContextMut;

use crate::{
    companion::state::HostInstanceState,
    prelude::*,
    setup::Setup,
    wasm::{connection::Connection, engine::Engine, module::StatefulModule},
};

mod abi;
pub mod state;

/// 🚀 The Companion engine.
pub struct Companion {
    connections: HashMap<String, StatefulModule>,
}

impl Companion {
    #[instrument(skip_all)]
    pub async fn from_setup(setup: Setup) -> Result<Self> {
        info!("loading connections…");

        let engine = Engine::new_async()?;
        let mut linker = engine.new_linker();
        abi::add_to(linker.as_mut())?;

        let connections: HashMap<String, StatefulModule> = {
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
                    let mut store = engine.new_store(HostInstanceState::for_connection(&id, ()));
                    let instance =
                        linker.instantiate_async(store.as_context_mut(), &module).await?;
                    let state = Connection::from(instance)
                        .init_async(store.as_context_mut(), settings)
                        .await?;
                    let stateful_module = StatefulModule { module, state };
                    Ok((id, stateful_module))
                })
                .try_collect()
                .await?
        };

        info!(n_connections = connections.len(), "completed");
        Ok(Self { connections })
    }
}

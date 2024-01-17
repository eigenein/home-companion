use std::collections::HashMap;

use futures::{stream, StreamExt, TryStreamExt};

use crate::{
    prelude::*,
    setup::Setup,
    wasm::{engine::Engine, module::Module},
};

/// 🚀 The Companion engine.
pub struct Companion {}

impl Companion {
    #[instrument(skip_all)]
    pub async fn from_setup(setup: &Setup) -> Result<Self> {
        info!("loading…");
        let engine = Engine::new_async()?;
        let linker = engine.new_linker();
        let _modules: HashMap<String, Module> = stream::iter(setup.connections.iter())
            .then(|(id, connection)| async {
                Ok::<_, Error>((id.to_string(), engine.load_module(&connection.module_path)?))
            })
            .and_then(|(id, module)| {
                let engine = &engine;
                let linker = &linker;
                async move {
                    let mut store = engine.new_store(());
                    let instance = linker.instantiate_async(&mut store, &module).await?;
                    instance.call_init_async(&mut store).await?;
                    Ok((id, module)) // TODO
                }
            })
            .try_collect()
            .await?;
        Ok(Self {})
    }
}

use std::{collections::HashMap, path::Path};

use futures::{stream, StreamExt};
use wasmtime::{Config, Instance, Module, Store, TypedFunc};

use crate::{prelude::*, setup::Setup};

/// Plugin initialization function type.
type InitFunc = TypedFunc<(), ()>;

pub struct EngineSetup<'a> {
    setup: &'a Setup,
    engine: wasmtime::Engine,
}

impl<'a> EngineSetup<'a> {
    pub fn new(setup: &'a Setup) -> Result<Self> {
        let mut config = Config::new();
        config.async_support(true);
        Ok(Self {
            setup,
            engine: wasmtime::Engine::new(&config)?,
        })
    }

    /// Load the connection modules.
    #[instrument(skip_all)]
    pub fn load(mut self) -> LoadedEngine {
        let modules = self
            .setup
            .connections
            .iter()
            .filter_map(|(id, connection)| {
                info!(id, ?connection.module_path, "loading connection module…");
                self.load_module(&connection.module_path).map(|module| (id.to_string(), module))
            })
            .collect();
        LoadedEngine { engine: self.engine, modules }
    }

    /// Load module and return `Some(module)` if it is loaded successfully, and `None` otherwise.
    #[instrument(skip_all, fields(path = ?path))]
    fn load_module(&self, path: &Path) -> Option<Module> {
        match Module::from_file(&self.engine, path) {
            Ok(module) => Some(module),
            Err(error) => {
                error!("failed to load module from `{path:?}`: {error:#}");
                None
            }
        }
    }
}

/// ⌛️ Engine with the modules loaded and ready to run.
pub struct LoadedEngine {
    engine: wasmtime::Engine,
    modules: HashMap<String, Module>,
}

impl LoadedEngine {
    /// Initialize the engine.
    pub async fn init(self) -> Engine {
        let connections: HashMap<String, ()> = stream::iter(self.modules.iter())
            .filter_map(|(id, module)| async {
                let mut store = Store::new(&self.engine, ());
                let instance = Instance::new_async(&mut store, module, &[]).await.unwrap();
                let init_func: InitFunc = instance.get_typed_func(&mut store, "init").unwrap();
                init_func.call_async(&mut store, ()).await.unwrap();
                Some((id.to_string(), ()))
            })
            .collect()
            .await;
        Engine { engine: self.engine }
    }
}

/// 🔗 Initialized connection: module and corresponding persistent state.
pub struct Connection {
    module: Module,
    state: Box<[u8]>,
}

/// 🚀 Finally, the main Companion engine.
pub struct Engine {
    engine: wasmtime::Engine,
}

impl Engine {
    /// Run the Companion engine indefinitely.
    pub async fn run(&self) -> Result {
        info!("running…");
        Ok(())
    }
}

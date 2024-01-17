mod cli;
mod engine;
mod prelude;
mod setup;
mod tracing;

use clap::Parser;

use crate::{
    cli::{Cli, Command},
    engine::EngineSetup,
    prelude::*,
    setup::Setup,
};

#[tokio::main]
async fn main() -> Result {
    let dotenv_result = dotenvy::dotenv();
    let cli = Cli::parse();
    let _tracing_guards = tracing::init(cli.sentry_dsn.as_deref(), cli.traces_sample_rate)?;
    if let Err(error) = dotenv_result {
        warn!("`.env` was not loaded: {error}");
    }
    match cli.command {
        Command::Run { setup_path } => {
            let setup = Setup::from_file(&setup_path)?;
            EngineSetup::new(&setup)?
                .load()
                .init()
                .await
                .run()
                .await
                .context("the engine has crashed")
        }
    }
}

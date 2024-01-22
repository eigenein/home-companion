mod cli;
mod companion;
mod helpers;
mod prelude;
mod setup;
mod tracing;
mod wasm;

use clap::Parser;

use crate::{
    cli::{Cli, Command},
    companion::Companion,
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
            Companion::from_setup(setup)
                .await
                .context("failed to create a Companion engine")?;
            Ok(())
        }
    }
}

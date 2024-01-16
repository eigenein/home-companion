mod cli;
mod prelude;
mod tracing;

use clap::Parser;

use crate::{
    cli::{Cli, Command},
    prelude::*,
};

#[tokio::main]
async fn main() -> Result {
    if dotenvy::dotenv().is_err() {
        warn!("failed to load `.env`");
    }
    let cli = Cli::parse();
    match cli.command {
        Command::Run => todo!(),
    }
}

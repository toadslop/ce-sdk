use crate::cli::CliArgs;
use clap::Parser;
use color_eyre::eyre::Result;
use commands::{load_api::LoadSpec, Command};

mod cli;
mod client;
mod commands;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = CliArgs::parse();
    let result = match args.command {
        cli::Commands::Generate(_) => todo!(),
        cli::Commands::LoadApi(mut ctx) => LoadSpec.run(&mut ctx),
        cli::Commands::LoadObject(_) => todo!(),
    };

    if let Err(err) = result {
        eprintln!("Error: {:?}", err)
    }

    Ok(())
}

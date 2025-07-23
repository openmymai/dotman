mod actions;
mod cli;
mod config;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { dir } => actions::init(dir)?,
        Commands::Add { path } => actions::add(path)?,
        Commands::Link { force } => actions::link(force)?,
        Commands::Unlink => actions::unlink()?,
        Commands::List => actions::list()?,
    }

    Ok(())
}
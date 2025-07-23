use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "A simple, fast, and reliable dotfiles manager.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init {
        #[arg(short, long)]
        dir: Option<PathBuf>,
    },

    Add {
        #[arg(required = true)]
        path: PathBuf,
    },

    Link {
        #[arg(long)]
        force: bool,
    },

    Unlink,

    List,
}
pub mod args;
pub mod read;
pub mod list;
pub mod stats;

use anyhow::Result;
use clap::Parser;

use args::{Cli, Command};

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Read(args) => read::run(args),
        Command::List => list::run(),
        Command::Stats(args) => stats::run(args),
    }
}

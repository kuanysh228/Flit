use anyhow::Result;

mod cli;
mod core;
mod readers;
mod ui;
mod storage;
mod config;
mod paths;
mod error;

fn main() -> Result<()> {
    cli::run()
}

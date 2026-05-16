use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "flit", version, about = "Modern RSVP terminal reader")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Read(ReadArgs),
    List,
    Stats(StatsArgs),
}

#[derive(Args)]
pub struct ReadArgs {
    pub file: Option<PathBuf>,

    #[arg(short = 'w', long, help = "Words per minute")]
    pub wpm: Option<u16>,

    #[arg(long, conflicts_with = "restart", help = "Resume from bookmark")]
    pub resume: bool,

    #[arg(long, conflicts_with = "resume", help = "Ignore bookmark and start from beginning")]
    pub restart: bool,

    #[arg(long, help = "Read from stdin")]
    pub stdin: bool,

    #[arg(short = 'r', long, help = "Start from word index (compatibility with original speedread)")]
    pub word_index: Option<u64>,
}

#[derive(Args)]
pub struct StatsArgs {
    #[arg(default_value = "all")]
    pub period: String,
}

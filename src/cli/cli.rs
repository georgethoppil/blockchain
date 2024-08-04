use clap::Parser;

use crate::Command;

/// Defines the command-line interface structure for the blockchain application
#[derive(Parser)]
#[command(name = "b")]
#[command(about = "Best blockchain")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

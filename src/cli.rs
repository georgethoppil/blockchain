use clap::Parser;

use crate::Command;

#[derive(Parser)]
#[command(name = "b")]
#[command(about = "Best blockchain")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

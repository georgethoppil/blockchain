use std::error::Error;

use clap::Parser;

use crate::{start_server, Command};

#[derive(Parser)]
#[command(name = "b")]
#[command(about = "Best blockchain")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

pub struct Config {
    pub command: Command,
}

impl Config {
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        match &self.command {
            Command::StartNode => {
                println!("Starting the blockchain node...");
                start_server().await;
            }
            Command::CreateAccount {
                id,
                starting_balance,
            } => {
                println!(
                    "Creating account with id: {} and starting balance: {}",
                    id, starting_balance
                );
            }
            Command::Transfer {
                from_account,
                to_account,
                amount,
            } => {
                println!(
                    "Transferring {} from {} to {}",
                    amount, from_account, to_account
                );
            }
            Command::Balance { account } => {
                println!("Getting balance for account: {}", account);
            }
        }
        Ok(())
    }
}

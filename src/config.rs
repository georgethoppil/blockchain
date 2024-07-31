use std::error::Error;

use clap::{Parser, Subcommand};

use crate::Blockchain;

#[derive(Parser)]
#[command(name = "b")]
#[command(about = "Best blockchain")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    StartNode,
    CreateAccount {
        id: String,
        starting_balance: u64,
    },
    Transfer {
        from_account: String,
        to_account: String,
        amount: u64,
    },
    Balance {
        account: String,
    },
}

pub struct Config {
    pub command: Commands,
    pub blockchain: Blockchain,
}

impl Config {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        match &self.command {
            Commands::StartNode => {
                println!("Starting the blockchain node...");
            }
            Commands::CreateAccount {
                id,
                starting_balance,
            } => {
                println!(
                    "Creating account with id: {} and starting balance: {}",
                    id, starting_balance
                );
            }
            Commands::Transfer {
                from_account,
                to_account,
                amount,
            } => {
                println!(
                    "Transferring {} from {} to {}",
                    amount, from_account, to_account
                );
            }
            Commands::Balance { account } => {
                println!("Getting balance for account: {}", account);
            }
        }
        Ok(())
    }
}

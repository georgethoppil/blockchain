use std::error::Error;

use clap::Parser;

use crate::{Client, Command, Server};

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
                Server::start_node().await;
            }
            Command::CreateAccount { id, balance } => {
                println!(
                    "Creating account with id: {} and starting balance: {}",
                    id, balance
                );

                let _ = Client::run_command(Command::CreateAccount {
                    id: id.clone(),
                    balance: balance.clone(),
                })
                .await?;
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
                let _ = Client::run_command(Command::Transfer {
                    from_account: from_account.clone(),
                    to_account: to_account.clone(),
                    amount: amount.clone(),
                })
                .await?;
            }
            Command::Balance { account } => {
                println!("Getting balance for account: {}", account);
                let _ = Client::run_command(Command::Balance {
                    account: account.clone(),
                })
                .await?;
            }
            _ => {
                println!("invalid command");
            }
        }
        Ok(())
    }
}

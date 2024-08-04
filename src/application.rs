use std::error::Error;

use crate::{Client, Command, Configuration, Server};

/// Represents the main application that coordinates blockchain operations
#[derive(Debug, Clone)]
pub struct Application {
    pub config: Configuration,
}

impl Application {
    /// Builds a new Application instance with the given configuration
    pub fn build(configuration: Configuration) -> Self {
        Application {
            config: configuration,
        }
    }
    /// Runs the appropriate command based on user input
    pub async fn run(&self, command: &Command) -> Result<(), Box<dyn Error>> {
        match command {
            Command::StartNode => {
                tracing::debug!("Starting the blockchain node...");
                Server::build(self.config.clone()).start_node().await?;
            }
            Command::CreateAccount { id, balance } => {
                tracing::debug!(
                    "Creating account with id: {} and starting balance: {}",
                    id,
                    balance
                );

                let _ = Client::build(self.config.clone())
                    .run_command(Command::CreateAccount {
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
                tracing::debug!(
                    "Transferring {} from {} to {}",
                    amount,
                    from_account,
                    to_account
                );
                let _ = Client::build(self.config.clone())
                    .run_command(Command::Transfer {
                        from_account: from_account.clone(),
                        to_account: to_account.clone(),
                        amount: amount.clone(),
                    })
                    .await?;
            }
            Command::Balance { account } => {
                tracing::debug!("Getting balance for account: {}", account);
                let _ = Client::build(self.config.clone())
                    .run_command(Command::Balance {
                        account: account.clone(),
                    })
                    .await?;
            }
            _ => {
                tracing::debug!("invalid command");
            }
        }
        Ok(())
    }
}

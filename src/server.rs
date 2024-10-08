use crate::{Blockchain, Command, CommandCodec, Configuration};
use futures::{SinkExt, StreamExt};
use std::{error::Error, sync::Arc, time::Duration};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
    time::interval,
};
use tokio_util::codec::Framed;

/// Shared blockchain state protected by a mutex
type Db = Arc<Mutex<Blockchain>>;

/// Represents the blockchain server
pub struct Server {
    host: String,
    port: u16,
    mining_timeout: u64,
}

impl Server {
    /// Builds a new Server instance with the given configuration
    pub fn build(config: Configuration) -> Self {
        Server {
            host: config.application.host,
            port: config.application.port,
            mining_timeout: config.application.mining_timeout,
        }
    }

    /// Starts the blockchain node, listening for incoming connections and processing blocks
    pub async fn start_node(&self) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port))
            .await
            .unwrap();
        let blockchain = Arc::new(Mutex::new(Blockchain::new()));
        tracing::debug!("starting server");

        // Start mining interval to add block to the chain and process pending transactions
        let db_clone = blockchain.clone();
        let mining_timeout = self.mining_timeout;
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(mining_timeout));
            loop {
                interval.tick().await;
                if let Ok(mut blockchain) = db_clone.try_lock() {
                    blockchain.add_block();
                } else {
                    tracing::debug!("problem accessing blockchain node in interval");
                }
            }
        });

        // Handle incoming socket requests
        loop {
            let (socket, _) = listener.accept().await?;
            let db = blockchain.clone();
            tokio::spawn(async move {
                Self::process(socket, db).await;
            });
        }
    }

    /// Processes incoming client connections and commands
    async fn process(socket: TcpStream, db: Db) {
        let mut framed = Framed::new(socket, CommandCodec);
        tracing::debug!("processing socket connection");
        while let Some(Ok(command)) = framed.next().await {
            let mut blockchain = match db.try_lock() {
                Ok(guard) => guard,
                Err(_) => {
                    tracing::error!("Failed to acquire lock on blockchain");
                    return;
                }
            };

            let message = match command {
                Command::CreateAccount { id, balance } => blockchain
                    .create_account(id, balance)
                    .unwrap_or_else(|| "Account creation is added to the blockchain".to_string()),
                Command::Transfer {
                    from_account,
                    to_account,
                    amount,
                } => blockchain
                    .transfer(from_account, to_account, amount)
                    .unwrap_or_else(|| "The transfer is added to the blockchain".to_string()),
                Command::Balance { account } => blockchain
                    .get_balance(&account)
                    .map(|balance| format!("balance is {}", balance))
                    .unwrap_or_else(|| "Account not found".to_string()),

                _ => "invalid command to process".to_string(),
            };

            if let Err(e) = framed.send(Command::Ack { message }).await {
                tracing::error!("Failed to send Ack: {:?}", e);
                break;
            }
        }
    }
}

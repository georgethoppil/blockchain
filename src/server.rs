use crate::{Blockchain, Command, CommandCodec};
use futures::{SinkExt, StreamExt};
use std::{sync::Arc, time::Duration};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
    time::interval,
};
use tokio_util::codec::Framed;

type Db = Arc<Mutex<Blockchain>>;

pub struct Server;

impl Server {
    pub async fn start_node() {
        let listener = TcpListener::bind("127.0.0.1:6370").await.unwrap();
        let blockchain = Arc::new(Mutex::new(Blockchain::new()));
        println!("starting server");

        // start 10 second interval to add block to the chain and process pending transactions
        let db_clone = blockchain.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                if let Ok(mut blockchain) = db_clone.try_lock() {
                    blockchain.add_block();
                } else {
                    println!("problem accessing blockchain node in interval");
                }
            }
        });

        //handle incoming socket requests
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let db = blockchain.clone();
            tokio::spawn(async move {
                Self::process(socket, db).await;
            });
        }
    }

    async fn process(socket: TcpStream, db: Db) {
        let mut framed = Framed::new(socket, CommandCodec);
        println!("processing socket connection");
        while let Some(Ok(command)) = framed.next().await {
            let mut blockchain = match db.try_lock() {
                Ok(guard) => guard,
                Err(_) => {
                    eprintln!("Failed to acquire lock on blockchain");
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
                eprintln!("Failed to send Ack: {:?}", e);
                break;
            }
        }
    }
}

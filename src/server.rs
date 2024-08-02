use crate::{Blockchain, Command, CommandCodec};
use futures::StreamExt;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    net::{TcpListener, TcpStream},
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
                if let Ok(mut blockchain) = db_clone.lock() {
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
            match command {
                Command::CreateAccount { id, balance } => {
                    println!("creating account with values {id} {balance}");
                    if let Ok(mut blockchain) = db.lock() {
                        blockchain.create_account(id, balance);
                    } else {
                        println!("problem accessing blockchain node for create account in socket processing");
                    }
                }
                Command::Transfer {
                    from_account,
                    to_account,
                    amount,
                } => {
                    println!(
                        "transfer fund details from {from_account} to {to_account} for ${amount}"
                    );
                    if let Ok(mut blockchain) = db.lock() {
                        blockchain.transfer(from_account, to_account, amount);
                    } else {
                        println!("problem accessing blockchain node for transfering amounts in socket processing");
                    }
                }
                Command::Balance { account } => {
                    println!("balance for {account}");
                    if let Ok(blockchain) = db.lock() {
                        if let Some(balance) = blockchain.get_balance(&account) {
                            println!("Account has $ {balance}");
                        } else {
                            println!("Account not found");
                        }
                    } else {
                        println!(
                            "problem accessing blockchain node for balance in socket processing"
                        );
                    }
                }

                _ => {
                    println!("invalid command");
                }
            };
        }
    }
}

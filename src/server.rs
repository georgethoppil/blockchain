use std::sync::{Arc, Mutex};

use tokio::net::{TcpListener, TcpStream};

use tokio_util::codec::Framed;

use crate::{Blockchain, Command, CommandCodec};

use futures::StreamExt;

type Db = Arc<Mutex<Blockchain>>;

pub struct Server;

impl Server {
    pub async fn start_node() {
        let listener = TcpListener::bind("127.0.0.1:6370").await.unwrap();
        let blockchain = Arc::new(Mutex::new(Blockchain::new()));
        println!("starting server");
        loop {
            // The second item contains the IP and port of the new connection.
            let (socket, _) = listener.accept().await.unwrap();
            let blockchain = blockchain.clone();
            tokio::spawn(async move {
                Self::process(socket, blockchain).await;
            });
        }
    }

    async fn process(socket: TcpStream, db: Db) {
        let mut framed = Framed::new(socket, CommandCodec);
        println!("processing socket connection");
        while let Some(Ok(command)) = framed.next().await {
            match command {
                Command::CreateAccount { id, balance } => {
                    // let mut db = db.lock();
                    // db.create_account(id, balance);
                    println!("got create account with values {id} {balance}");
                }
                Command::Transfer {
                    from_account,
                    to_account,
                    amount,
                } => {
                    // let mut db = db.lock();
                    // db.transfer(from, to, amount);
                    println!("got transfer {from_account} {to_account} {amount}");
                }
                Command::Balance { account } => {
                    // let db = db.lock();
                    // if let Some(balance) = db.get_balance(&account) {
                    //     balance.to_string()
                    // } else {
                    //     "Account not found".to_string()
                    // }
                    println!("balance for {account}");
                }

                _ => {
                    println!("invalid command");
                }
            };
        }
    }
}

use std::sync::{Arc, Mutex};

use mini_redis::Connection;
use tokio::net::{TcpListener, TcpStream};

use crate::Blockchain;

type Db = Arc<Mutex<Blockchain>>;

pub async fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:6370").await.unwrap();
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        let blockchain = blockchain.clone();
        tokio::spawn(async move {
            process(socket, blockchain).await;
        });
    }
}

async fn process(socket: TcpStream, blockchain: Db) {
    let mut connection = Connection::new(socket);
}

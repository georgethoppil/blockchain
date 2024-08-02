use std::error::Error;

use futures::SinkExt;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

use crate::{Command, CommandCodec};

pub struct Client;

impl Client {
    pub async fn run_command(command: Command) -> Result<(), Box<dyn Error>> {
        let stream = TcpStream::connect("127.0.0.1:6370").await?;
        let mut framed = Framed::new(stream, CommandCodec);
        match framed.send(command).await {
            Ok(_) => println!("success"),
            Err(err) => println!("errored out {err}"),
        }
        Ok(())
    }
}

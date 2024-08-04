use std::error::Error;

use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

use crate::{Command, CommandCodec, Configuration};

pub struct Client {
    host: String,
    port: u16,
}

impl Client {
    pub fn build(config: Configuration) -> Self {
        Client {
            host: config.application.host,
            port: config.application.port,
        }
    }
    pub async fn run_command(&self, command: Command) -> Result<(), Box<dyn Error>> {
        let stream = TcpStream::connect(format!("{}:{}", self.host, self.port)).await?;
        let mut framed = Framed::new(stream, CommandCodec);
        framed.send(command).await?;

        if let Some(Ok(response)) = framed.next().await {
            match response {
                Command::Ack { message } => {
                    println!("Ack: {}", message);
                }
                _ => {
                    println!("Received unexpected response");
                }
            }
        }
        Ok(())
    }
}

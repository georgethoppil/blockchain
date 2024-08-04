use std::error::Error;

use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

use crate::{Command, CommandCodec, Configuration};

/// Represents a client that can interact with the blockchain server
pub struct Client {
    host: String,
    port: u16,
}

impl Client {
    /// Builds a new Client instance with the given configuration
    pub fn build(config: Configuration) -> Self {
        Client {
            host: config.application.host,
            port: config.application.port,
        }
    }

    /// Runs a command by sending it to the server and processing the response
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

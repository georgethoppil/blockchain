use std::io;

use bytes::{BufMut, BytesMut};
use clap::Subcommand;

use serde::{Deserialize, Serialize};
use tokio_util::codec::{Decoder, Encoder};

/// Represents the various commands that can be executed in the blockchain application
#[derive(Subcommand, Serialize, Deserialize, Debug)]
pub enum Command {
    /// Start the blockchain node
    StartNode,
    /// Create a new account
    CreateAccount {
        #[clap(help = "ID of the account")]
        id: String,
        #[clap(help = "Starting balance of the account", value_parser = greater_than_zero)]
        balance: u64,
    },
    /// Transfer funds between accounts
    Transfer {
        #[clap(help = "ID of the source account")]
        from_account: String,
        #[clap(help = "ID of the destination account")]
        to_account: String,
        #[clap(help = "Amount to transfer", value_parser = greater_than_zero)]
        amount: u64,
    },
    /// Get the balance of an account
    Balance {
        #[clap(help = "ID of the account to query")]
        account: String,
    },
    /// Acknowledgment command
    #[clap(skip)]
    Ack { message: String },
}

/// Validates that the input is a positive number
fn greater_than_zero(val: &str) -> Result<u64, String> {
    let amount: u64 = val
        .parse()
        .map_err(|_| format!("'{}' is not a valid number", val))?;
    if amount > 0 {
        Ok(amount)
    } else {
        Err(format!("Amount must be greater than 0, but got '{}'", val))
    }
}

/// Codec for serializing and deserializing Command objects
pub struct CommandCodec;

impl Encoder<Command> for CommandCodec {
    type Error = io::Error;

    /// Encodes a Command into bytes
    fn encode(&mut self, item: Command, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let bytes =
            serde_json::to_vec(&item).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        dst.put_slice(&bytes);
        Ok(())
    }
}

impl Decoder for CommandCodec {
    type Item = Command;
    type Error = io::Error;

    /// Decodes bytes into a Command
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }
        let command: Command =
            serde_json::from_slice(&src).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        src.clear();
        Ok(Some(command))
    }
}

use std::io;

use bytes::{BufMut, BytesMut};
use clap::Subcommand;

use serde::{Deserialize, Serialize};
use tokio_util::codec::{Decoder, Encoder};

#[derive(Subcommand, Serialize, Deserialize, Debug)]
pub enum Command {
    StartNode,
    CreateAccount {
        id: String,
        balance: u64,
    },
    Transfer {
        from_account: String,
        to_account: String,
        amount: u64,
    },
    Balance {
        account: String,
    },
}

pub struct CommandCodec;

impl Encoder<Command> for CommandCodec {
    type Error = io::Error;

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

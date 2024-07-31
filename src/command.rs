use clap::Subcommand;
use mini_redis::Frame;

use crate::parse::Parse;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Subcommand)]
pub enum Command {
    StartNode,
    CreateAccount {
        id: String,
        starting_balance: u64,
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

impl Command {
    pub fn from_frame(frame: Frame) -> Result<Command> {
        let mut parse = Parse::new(frame)?;
        let command_name = parse.next_string()?.to_lowercase();
        let command = match &command_name[..] {
            // "get" => Command::Get(Get::parse_frames(&mut parse)?),
            // "publish" => Command::Publish(Publish::parse_frames(&mut parse)?),
            // "set" => Command::Set(Set::parse_frames(&mut parse)?),
            // "subscribe" => Command::Subscribe(Subscribe::parse_frames(&mut parse)?),
            // "unsubscribe" => Command::Unsubscribe(Unsubscribe::parse_frames(&mut parse)?),
            // _ => {
            //     // The command is not recognized and an Unknown command is
            //     // returned.
            //     //
            //     // `return` is called here to skip the `finish()` call below. As
            //     // the command is not recognized, there is most likely
            //     // unconsumed fields remaining in the `Parse` instance.
            //     return Ok(Command::Unknown(Unknown::new(command_name)));
            // }
            _ => return Ok(Command::StartNode),
        };
        parse.finish()?;
        Ok(command)
    }
}

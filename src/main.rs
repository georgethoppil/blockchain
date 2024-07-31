use std::error::Error;

use b::{Cli, Config};
use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let config = Config {
        command: cli.command,
    };

    match config.run() {
        Err(err) => {
            println!("something went wrong {}", err);
            Err(err)
        }
        _ => Ok(()),
    }
    /*
       1. We want to parse cli inputs for
         - start node
         - balance
         - transfer
         - create account

         some errors to define.
         1) cant do anything unless server is up
         2) if no account is found, we say account not found
         3) cant transfer negative amount
         4) cant create over existing account (can use uuid here)

        2. Once server is started, we will clone/create a new block/last block and start it with a ticker
        3. We will create a mpsc with a feedback to handle the balance, transfer, create account
        4. Once the ticker is up, we will commit/mint the block to the server state and start 2. again
        5. Need to add logging

    */
}

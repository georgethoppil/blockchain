use std::error::Error;

use b::{Cli, Config};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let config = Config {
        command: cli.command,
    };

    match config.run().await {
        Err(err) => {
            println!("something went wrong {}", err);
            Err(err)
        }
        _ => Ok(()),
    }
    /*


      2. add config/
      2.5 rename some class files
       3. Add proper tracing

       5. Add test cases
       6. Add readme + proper instructions

    */
}

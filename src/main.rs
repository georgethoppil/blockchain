use std::error::Error;

use b::{get_configuration, Application, Cli};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let configuration = get_configuration().expect("Failed to read configuration");

    match Application::build(configuration).run(&cli.command).await {
        Err(err) => {
            println!("something went wrong {}", err);
            Err(err)
        }
        _ => Ok(()),
    }
    /*



       3. Add proper tracing

       5. Add test cases
       6. Add readme + proper instructions

    */
}

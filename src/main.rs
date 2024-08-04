use std::error::Error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use b::{get_configuration, Application, Cli};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "b=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let cli = Cli::parse();
    let configuration = get_configuration().expect("Failed to read configuration");
    tracing::debug!("config is {:?}", configuration);
    match Application::build(configuration).run(&cli.command).await {
        Err(err) => {
            tracing::error!("something went wrong {}", err);
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

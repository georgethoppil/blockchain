#[cfg(test)]
mod integration_tests {
    use b::{AppConfiguration, Client, Command, Configuration, Server};
    use tokio;

    #[tokio::test]
    async fn test_client_server_interaction() {
        // Start server
        let config = Configuration {
            application: AppConfiguration {
                host: "127.0.0.1".to_string(),
                port: 8080,
                mining_timeout: 5,
            },
        };
        let server = Server::build(config.clone());
        tokio::spawn(async move {
            server.start_node().await.unwrap();
        });

        // Give the server some time to start
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Create client
        let client = Client::build(config);

        // Test account creation
        let create_result = client
            .run_command(Command::CreateAccount {
                id: "alice".to_string(),
                balance: 100,
            })
            .await;
        assert!(create_result.is_ok());

        // Test balance check
        let balance_result = client
            .run_command(Command::Balance {
                account: "alice".to_string(),
            })
            .await;
        assert!(balance_result.is_ok());

        // Test transfer
        let transfer_result = client
            .run_command(Command::Transfer {
                from_account: "alice".to_string(),
                to_account: "bob".to_string(),
                amount: 50,
            })
            .await;
        assert!(transfer_result.is_ok());

        tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;

        // Check final balance
        let final_balance_result = client
            .run_command(Command::Balance {
                account: "alice".to_string(),
            })
            .await;
        assert!(final_balance_result.is_ok());
    }
}

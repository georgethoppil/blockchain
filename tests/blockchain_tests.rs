#[cfg(test)]
mod tests {
    use b::Blockchain;

    #[test]
    fn test_new_blockchain() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.chain.len(), 1);
        assert!(blockchain.accounts.is_empty());
        assert!(blockchain.pending_transactions.is_empty());
    }

    #[test]
    fn test_create_account() {
        let mut blockchain = Blockchain::new();
        let result = blockchain.create_account("alice".to_string(), 100);
        assert!(result.is_none());

        blockchain.add_block();

        let balance = blockchain.get_balance("alice");
        assert_eq!(balance, Some(100));
    }

    #[test]
    fn test_transfer() {
        let mut blockchain = Blockchain::new();
        blockchain.create_account("alice".to_string(), 100);
        blockchain.create_account("bob".to_string(), 50);
        blockchain.add_block();

        let result = blockchain.transfer("alice".to_string(), "bob".to_string(), 30);
        assert!(result.is_none());

        blockchain.add_block();

        assert_eq!(blockchain.get_balance("alice"), Some(70));
        assert_eq!(blockchain.get_balance("bob"), Some(80));
    }

    #[test]
    fn test_insufficient_balance() {
        let mut blockchain = Blockchain::new();
        blockchain.create_account("alice".to_string(), 100);
        blockchain.create_account("bob".to_string(), 50);
        blockchain.add_block();

        let result = blockchain.transfer("alice".to_string(), "bob".to_string(), 150);
        assert_eq!(
            result,
            Some("Insufficient amount in the account".to_string())
        );

        blockchain.add_block();

        assert_eq!(blockchain.get_balance("alice"), Some(100));
        assert_eq!(blockchain.get_balance("bob"), Some(50));
    }

    #[test]
    fn test_nonexistent_account() {
        let mut blockchain = Blockchain::new();
        blockchain.create_account("alice".to_string(), 100);
        blockchain.add_block();

        let result = blockchain.transfer("alice".to_string(), "bob".to_string(), 50);
        assert_eq!(result, Some("Account doesn't exist".to_string()));
    }

    #[test]
    fn test_create_existing_account() {
        let mut blockchain = Blockchain::new();
        blockchain.create_account("alice".to_string(), 100);
        blockchain.add_block();

        let result = blockchain.create_account("alice".to_string(), 50);
        assert_eq!(result, Some("Account already exists".to_string()));

        blockchain.add_block();

        assert_eq!(blockchain.get_balance("alice"), Some(100));
    }
}

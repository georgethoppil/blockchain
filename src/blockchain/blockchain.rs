use std::collections::HashMap;

use super::Transaction;
/// Represents a block in the blockchain
#[derive(Debug, Clone)]
pub struct Block {
    pub transactions: Vec<Transaction>,
}

impl Block {
    /// Creates a new block with the given transactions
    fn new(transactions: Vec<Transaction>) -> Self {
        Block { transactions }
    }
}

/// Represents the blockchain and manages accounts and transactions
#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub accounts: HashMap<String, u64>,
    pub pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    /// Creates a new blockchain with a genesis block
    pub fn new() -> Self {
        let genesis_block = Block::new(vec![]);

        Blockchain {
            chain: vec![genesis_block],
            accounts: HashMap::new(),
            pending_transactions: Vec::new(),
        }
    }

    /// Adds a new block to the chain, processing pending transactions
    pub fn add_block(&mut self) {
        if self.pending_transactions.len() > 0 {
            tracing::debug!("Adding pending transcations to blockchain");
            let new_block = Block::new(self.pending_transactions.clone());
            Self::process_pending_transactions(self);
            self.chain.push(new_block);
            self.pending_transactions.clear();
            tracing::debug!("State of blockchain {:#?}", self);
        } else {
            tracing::debug!("No pending transactions to add to blockchain");
        }
    }

    /// Retrieves the balance of a given account
    pub fn get_balance(&self, account: &str) -> Option<u64> {
        self.accounts.get(account).cloned()
    }

    /// Initiates a transfer between two accounts
    pub fn transfer(
        &mut self,
        from_account: String,
        to_account: String,
        amount: u64,
    ) -> Option<String> {
        // Check if both accounts exist
        if !self.accounts.contains_key(&from_account) || !self.accounts.contains_key(&to_account) {
            return Some("Account doesn't exist".to_string());
        }

        // Check if there are sufficient funds
        if self.accounts[&from_account] < amount {
            return Some("Insufficient amount in the account".to_string());
        }

        let transaction = Transaction {
            from: Some(from_account),
            to: Some(to_account),
            amount,
            account_id: None,
        };
        self.pending_transactions.push(transaction);
        None
    }

    /// Creates a new account with a starting balance
    pub fn create_account(&mut self, id: String, starting_balance: u64) -> Option<String> {
        if self.accounts.contains_key(&id) {
            return Some("Account already exists".to_string());
        }

        let transaction = Transaction {
            from: None,
            to: None,
            amount: starting_balance,
            account_id: Some(id),
        };
        self.pending_transactions.push(transaction);
        None
    }

    /// Processes all pending transactions
    fn process_pending_transactions(&mut self) {
        for transaction in &self.pending_transactions {
            if transaction.account_id.is_some() {
                // Account creation
                if let Some(account_id) = &transaction.account_id {
                    self.accounts
                        .insert(account_id.to_string(), transaction.amount);
                }
            } else if let (Some(from), Some(to)) = (&transaction.from, &transaction.to) {
                // Fund transfer
                if self.accounts.contains_key(from) && self.accounts.contains_key(to) {
                    if self.accounts[from] >= transaction.amount {
                        *self.accounts.get_mut(from).unwrap() -= transaction.amount;
                        *self.accounts.get_mut(to).unwrap() += transaction.amount;
                    }
                }
            }
        }
    }
}

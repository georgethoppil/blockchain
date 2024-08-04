use std::collections::HashMap;

use super::Transaction;
#[derive(Debug, Clone)]

struct Block {
    transactions: Vec<Transaction>,
}

impl Block {
    fn new(transactions: Vec<Transaction>) -> Self {
        Block { transactions }
    }
}
#[derive(Debug, Clone)]
pub struct Blockchain {
    chain: Vec<Block>,
    accounts: HashMap<String, u64>,
    pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(vec![]);

        Blockchain {
            chain: vec![genesis_block],
            accounts: HashMap::new(),
            pending_transactions: Vec::new(),
        }
    }

    pub fn add_block(&mut self) {
        if self.pending_transactions.len() > 0 {
            println!("Adding pending transcations to blockchain");
            let new_block = Block::new(self.pending_transactions.clone());
            Self::process_pending_transactions(self);
            self.chain.push(new_block);
            self.pending_transactions.clear();
            println!("State of blockchain {:#?}", self);
        } else {
            println!("No pending transactions to add to blockchain");
        }
    }

    pub fn get_balance(&self, account: &str) -> Option<u64> {
        self.accounts.get(account).cloned()
    }

    pub fn transfer(
        &mut self,
        from_account: String,
        to_account: String,
        amount: u64,
    ) -> Option<String> {
        // check if both accounts exist
        if !self.accounts.contains_key(&from_account) || !self.accounts.contains_key(&to_account) {
            return Some("account doesn't exist".to_string());
        }

        // if there is sufficient amounts
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

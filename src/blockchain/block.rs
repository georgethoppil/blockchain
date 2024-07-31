use std::{collections::HashMap, thread, time::Duration};

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

    fn add_block(&mut self) {
        println!("Adding pending transcations to blockchain");
        let new_block = Block::new(self.pending_transactions.clone());
        self.chain.push(new_block);
    }
}

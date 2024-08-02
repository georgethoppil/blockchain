#[derive(Debug, Clone)]
pub struct Transaction {
    pub from: Option<String>,       // None for account creation
    pub to: Option<String>,         // None for account creation
    pub amount: u64,                // Balance for account creation
    pub account_id: Option<String>, // Some for account creation
}

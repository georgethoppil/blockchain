#[derive(Debug, Clone)]
pub struct Transaction {
    from: Option<String>,       // None for account creation
    to: Option<String>,         // None for account creation
    amount: u64,                // Balance for account creation
    account_id: Option<String>, // Some for account creation
}

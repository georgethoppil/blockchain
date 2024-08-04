#[derive(Debug, Clone)]
pub struct Transaction {
    /// The account sending funds (None for account creation)
    pub from: Option<String>,
    /// The account receiving funds (None for account creation)
    pub to: Option<String>,
    /// The amount of funds transferred or initial balance for account creation
    pub amount: u64,
    /// The ID of the account being created (Some for account creation, None for transfers)
    pub account_id: Option<String>,
}

use serde::{Deserialize, Serialize};

/// Represents a complete transaction. All fields must be filled (no nulls).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    /// Date of the transaction as a timestamp (milliseconds since epoch)
    pub date: i64,
    /// Index for the transaction for date (allows balance-safe ordering)
    pub index: usize,
    /// Description of the transaction
    pub description: String,
    /// Amount of the transaction
    pub amount: f64,
    /// Balance after the transaction
    pub balance: f64,
    /// Account number the transaction belongs to
    pub account_number: String,
}

impl Transaction {
    pub fn new(
        date: i64,
        index: usize,
        description: String,
        amount: f64,
        balance: f64,
        account_number: String,
    ) -> Self {
        Self {
            date,
            index,
            description,
            amount,
            balance,
            account_number,
        }
    }

    /// Identity of a transaction. The description is deliberately excluded and
    /// amounts are compared at cent precision.
    pub fn identity(&self) -> (i64, usize, i64, i64, &str) {
        (
            self.date,
            self.index,
            (self.amount * 100.0).round() as i64,
            (self.balance * 100.0).round() as i64,
            self.account_number.as_str(),
        )
    }
}

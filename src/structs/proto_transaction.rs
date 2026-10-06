use crate::structs::transaction::Transaction;
use regex::Regex;
use serde::{Deserialize, Serialize};
/// Represents an incomplete transaction.
/// Serves as a temporary structure to hold transaction data before it is fully parsed, validated, and filled.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProtoTransaction {
    /// Date of the transaction as a timestamp (milliseconds since epoch)
    pub date: Option<i64>,
    /// Index for the transaction for date (allows balance-safe ordering)
    pub index: usize,
    /// Description of the transaction
    pub description: String,
    /// Amount of the transaction
    pub amount: Option<f64>,
    /// Balance after the transaction
    pub balance: Option<f64>,
}

impl ProtoTransaction {
    /// Create a new ProtoTransaction.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns true if all required fields are present and description is not empty.
    pub fn is_ready(&self) -> bool {
        self.date.is_some()
            && self.amount.is_some()
            && self.balance.is_some()
            && !self.description.is_empty()
    }

    /// Converts to a Transaction if all fields are present.
    pub fn to_transaction(&self, account_number: &str) -> Result<Transaction, String> {
        if !self.is_ready() {
            return Err("Cannot convert to Transaction: fields are missing".to_string());
        }
        Ok(Transaction::new(
            self.date.unwrap(),
            self.index,
            self.description.clone(),
            self.amount.unwrap(),
            self.balance.unwrap(),
            account_number.to_string(),
        ))
    }

    /// Checks if all specified required fields are set.
    pub fn has_required_fields_set(&self, required_fields: &[String]) -> bool {
        for field in required_fields {
            match field.as_str() {
                "date" if self.date.is_none() => return false,
                "description" if self.description.is_empty() => return false,
                "amount" if self.amount.is_none() => return false,
                "balance" if self.balance.is_none() => return false,
                _ => {}
            }
        }
        true
    }

    /// Set the amount for this transaction.
    pub fn set_amount(&mut self, amount: f64) {
        self.amount = Some(amount);
    }

    /// Set the balance for this transaction.
    pub fn set_balance(&mut self, balance: f64) {
        self.balance = Some(balance);
    }

    /// Set the date for this transaction.
    pub fn set_date(&mut self, date: i64) {
        self.date = Some(date);
    }

    /// Set index for this transaction.
    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    /// Cleans the description by trimming whitespace and removing unwanted patterns.
    pub fn clean_description(&mut self, exclude_patterns: &[Regex]) {
        let mut desc = self.description.trim().to_string();
        for pattern in exclude_patterns {
            desc = pattern.replace_all(&desc, "").to_string();
        }
        self.description = desc.trim().to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_clean_description_trims_whitespace() {
        let mut tx = ProtoTransaction::new();
        tx.description = "  Payment to Store  ".to_string();

        tx.clean_description(&[]);

        assert_eq!(tx.description, "Payment to Store");
    }

    #[test]
    fn test_clean_description_removes_single_pattern() {
        let mut tx = ProtoTransaction::new();
        tx.description = "Payment to Store - REF123456".to_string();

        let patterns = vec![Regex::new(r" - REF\d+").unwrap()];
        tx.clean_description(&patterns);

        assert_eq!(tx.description, "Payment to Store");
    }

    #[test]
    fn test_clean_description_removes_multiple_patterns() {
        let mut tx = ProtoTransaction::new();
        tx.description = "Payment to Store - REF123456 | TXN789".to_string();

        let patterns = vec![
            Regex::new(r" - REF\d+").unwrap(),
            Regex::new(r" \| TXN\d+").unwrap(),
        ];
        tx.clean_description(&patterns);

        assert_eq!(tx.description, "Payment to Store");
    }

    #[test]
    fn test_clean_description_no_patterns_match() {
        let mut tx = ProtoTransaction::new();
        tx.description = "Payment to Store".to_string();

        let patterns = vec![Regex::new(r"NONEXISTENT").unwrap()];
        tx.clean_description(&patterns);

        assert_eq!(tx.description, "Payment to Store");
    }

    #[test]
    fn test_clean_description_empty_patterns() {
        let mut tx = ProtoTransaction::new();
        tx.description = "  Payment to Store  ".to_string();

        tx.clean_description(&[]);

        assert_eq!(tx.description, "Payment to Store");
    }

    #[test]
    fn test_clean_description_pattern_removes_entire_string() {
        let mut tx = ProtoTransaction::new();
        tx.description = "REF123456".to_string();

        let patterns = vec![Regex::new(r"REF\d+").unwrap()];
        tx.clean_description(&patterns);

        assert_eq!(tx.description, "");
    }

    #[test]
    fn test_clean_description_multiple_occurrences_same_pattern() {
        let mut tx = ProtoTransaction::new();
        tx.description = "Payment REF123 to Store REF456".to_string();

        let patterns = vec![Regex::new(r"REF\d+").unwrap()];
        tx.clean_description(&patterns);

        assert_eq!(tx.description, "Payment  to Store");
    }

    #[test]
    fn test_clean_description_overlapping_patterns() {
        let mut tx = ProtoTransaction::new();
        tx.description = "Payment ABC123DEF456".to_string();

        let patterns = vec![
            Regex::new(r"ABC\d+").unwrap(),
            Regex::new(r"DEF\d+").unwrap(),
        ];
        tx.clean_description(&patterns);

        assert_eq!(tx.description, "Payment");
    }

    #[test]
    fn test_clean_description_case_sensitive_patterns() {
        let mut tx = ProtoTransaction::new();
        tx.description = "Payment REF123 ref456".to_string();

        let patterns = vec![Regex::new(r"REF\d+").unwrap()]; // Case sensitive
        tx.clean_description(&patterns);

        assert_eq!(tx.description, "Payment  ref456");
    }

    #[test]
    fn test_clean_description_case_insensitive_patterns() {
        let mut tx = ProtoTransaction::new();
        tx.description = "Payment REF123 ref456".to_string();

        let patterns = vec![Regex::new(r"(?i)ref\d+").unwrap()]; // Case insensitive
        tx.clean_description(&patterns);

        assert_eq!(tx.description, "Payment");
    }

    #[test]
    fn test_clean_description_whitespace_cleanup_after_removal() {
        let mut tx = ProtoTransaction::new();
        tx.description = "  Payment   REF123   to   Store  ".to_string();

        let patterns = vec![Regex::new(r"REF\d+").unwrap()];
        tx.clean_description(&patterns);

        // Note: clean_description only trims leading/trailing whitespace, not internal whitespace
        assert_eq!(tx.description, "Payment      to   Store");
    }

    #[test]
    fn test_clean_description_special_regex_characters() {
        let mut tx = ProtoTransaction::new();
        tx.description = "Payment $100.50 (fee)".to_string();

        let patterns = vec![
            Regex::new(r"\$\d+\.\d+").unwrap(),   // Dollar amounts
            Regex::new(r"\s*\([^)]+\)").unwrap(), // Text in parentheses with optional leading space
        ];
        tx.clean_description(&patterns);

        assert_eq!(tx.description, "Payment");
    }

    #[test]
    fn test_clean_description_complex_real_world_example() {
        let mut tx = ProtoTransaction::new();
        tx.description = "  EFTPOS WDL MELBOURNE VIC 123456 07-11 14:35  ".to_string();

        let patterns = vec![
            Regex::new(r"\s+\d{6}\s+").unwrap(), // " 123456 " (space-6digits-space)
            Regex::new(r"\d{2}-\d{2}\s+\d{2}:\d{2}").unwrap(), // "07-11 14:35" (date-time pattern)
        ];
        tx.clean_description(&patterns);

        // After removing " 123456 " and "07-11 14:35", we get "EFTPOS WDL MELBOURNE VIC"
        assert_eq!(tx.description, "EFTPOS WDL MELBOURNE VIC");
    }

    #[test]
    fn test_clean_description_empty_input() {
        let mut tx = ProtoTransaction::new();
        tx.description = "".to_string();

        let patterns = vec![Regex::new(r"REF\d+").unwrap()];
        tx.clean_description(&patterns);

        assert_eq!(tx.description, "");
    }

    #[test]
    fn test_clean_description_whitespace_only_input() {
        let mut tx = ProtoTransaction::new();
        tx.description = "   \t\n   ".to_string();

        let patterns = vec![Regex::new(r"REF\d+").unwrap()];
        tx.clean_description(&patterns);

        assert_eq!(tx.description, "");
    }
}

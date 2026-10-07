use crate::structs::account_data::benchmark_report;
use crate::structs::{AccountData, Benchmark, Transaction};
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

/// Flattened, validated transactions pooled from one or more accounts.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StatementData {
    pub transactions: Vec<Transaction>,
    #[serde(skip)]
    pub benchmark: Benchmark,
}

impl StatementData {
    pub fn new() -> Self {
        Self::default()
    }

    /// Import the transactions of a complete, valid AccountData.
    /// Transactions already present (same date, index, amount, balance and
    /// account number) are skipped. Returns the number of transactions added.
    /// Nothing is imported if the AccountData is incomplete.
    pub fn import(&mut self, account_data: &AccountData) -> Result<usize, String> {
        let account_number = account_data
            .account_number
            .as_ref()
            .ok_or("Cannot import AccountData: missing account number")?;
        if !account_data.errors.is_empty() {
            return Err("Cannot import AccountData: it contains errors".to_string());
        }

        let mut incoming = Vec::with_capacity(account_data.proto_transactions.len());
        for proto_tx in &account_data.proto_transactions {
            if !proto_tx.is_ready() {
                return Err(format!(
                    "Incomplete transaction found: date={:?}, index={}, description='{}', amount={:?}, balance={:?}",
                    proto_tx.date,
                    proto_tx.index,
                    proto_tx.description,
                    proto_tx.amount,
                    proto_tx.balance
                ));
            }
            incoming.push(proto_tx.to_transaction(account_number)?);
        }

        let mut seen: HashSet<(i64, usize, i64, i64, String)> = self
            .transactions
            .iter()
            .map(|t| {
                let id = t.identity();
                (id.0, id.1, id.2, id.3, id.4.to_string())
            })
            .collect();
        let mut added = 0;
        for tx in incoming {
            let id = tx.identity();
            if seen.insert((id.0, id.1, id.2, id.3, id.4.to_string())) {
                self.transactions.push(tx);
                added += 1;
            }
        }
        Ok(added)
    }
}

impl fmt::Display for StatementData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::from("Statement Data:\n  Transactions:\n");
        for (i, tx) in self.transactions.iter().enumerate() {
            let date_str = DateTime::<Utc>::from_timestamp_millis(tx.date)
                .map(|dt| dt.format("%d %b %Y").to_string())
                .unwrap_or_else(|| tx.date.to_string());
            result.push_str(&format!(
                "    {}: {}, {}, \"{}\", {:.2}, {:.2}, {}\n",
                i + 1,
                tx.account_number,
                date_str,
                tx.description,
                tx.amount,
                tx.balance,
                tx.index
            ));
        }
        result.push_str(&benchmark_report(&self.benchmark));
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::ProtoTransaction;

    fn proto(date: i64, index: usize, amount: f64, balance: f64, desc: &str) -> ProtoTransaction {
        ProtoTransaction {
            date: Some(date),
            index,
            description: desc.to_string(),
            amount: Some(amount),
            balance: Some(balance),
        }
    }

    fn account(number: &str, txs: Vec<ProtoTransaction>) -> AccountData {
        let mut ad = AccountData::new();
        ad.set_account_number(number.to_string());
        ad.proto_transactions = txs;
        ad
    }

    #[test]
    fn import_pools_accounts_and_skips_duplicates() {
        let mut sd = StatementData::new();
        let a = account("A", vec![proto(1, 0, 10.0, 110.0, "x")]);
        let a_again = account(
            "A",
            vec![
                proto(1, 0, 10.0, 110.0, "different"),
                proto(1, 1, 5.0, 115.0, "y"),
            ],
        );
        let b = account("B", vec![proto(1, 0, 10.0, 110.0, "x")]);
        assert_eq!(sd.import(&a).unwrap(), 1);
        assert_eq!(sd.import(&a_again).unwrap(), 1);
        assert_eq!(sd.import(&b).unwrap(), 1);
        assert_eq!(sd.transactions.len(), 3);
    }

    #[test]
    fn import_rejects_incomplete_data_without_partial_import() {
        let mut sd = StatementData::new();
        let mut bad = proto(1, 1, 5.0, 115.0, "y");
        bad.balance = None;
        let ad = account("A", vec![proto(1, 0, 10.0, 110.0, "x"), bad]);
        assert!(sd.import(&ad).is_err());
        assert!(sd.transactions.is_empty());
        let mut no_number = AccountData::new();
        no_number.proto_transactions = vec![proto(1, 0, 10.0, 110.0, "x")];
        assert!(sd.import(&no_number).is_err());
    }
}

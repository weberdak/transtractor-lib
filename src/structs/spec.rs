use crate::configs::db::ConfigDB;
use crate::parsers::flows::text_items_to_statement_data::text_items_to_statement_data;
use crate::structs::StatementData;
use crate::structs::text_item::TextItem;
use serde::{Deserialize, Serialize};
use serde_json;

const FLOAT_MATCH_TOLERANCE: f64 = 0.01;

fn push_f64_field_diff(diffs: &mut Vec<String>, field_name: &str, generated: f64, expected: f64) {
    let delta = (generated - expected).abs();
    if delta >= FLOAT_MATCH_TOLERANCE {
        diffs.push(format!(
            "{} mismatch: generated {:.6}, expected {:.6}, |delta| {:.6} >= {:.6}",
            field_name, generated, expected, delta, FLOAT_MATCH_TOLERANCE
        ));
    }
}

pub fn diff(generated: &StatementData, expected: &StatementData) -> Vec<String> {
    let mut diffs = Vec::new();

    if generated.transactions.len() != expected.transactions.len() {
        diffs.push(format!(
            "transactions length mismatch: generated {}, expected {}",
            generated.transactions.len(),
            expected.transactions.len()
        ));
    }

    for (index, (g, e)) in generated
        .transactions
        .iter()
        .zip(expected.transactions.iter())
        .enumerate()
    {
        if g.date != e.date {
            diffs.push(format!(
                "transactions[{}].date mismatch: generated {:?}, expected {:?}",
                index, g.date, e.date
            ));
        }
        if g.index != e.index {
            diffs.push(format!(
                "transactions[{}].index mismatch: generated {}, expected {}",
                index, g.index, e.index
            ));
        }
        if g.description != e.description {
            diffs.push(format!(
                "transactions[{}].description mismatch: generated {:?}, expected {:?}",
                index, g.description, e.description
            ));
        }
        if g.account_number != e.account_number {
            diffs.push(format!(
                "transactions[{}].account_number mismatch: generated {:?}, expected {:?}",
                index, g.account_number, e.account_number
            ));
        }
        push_f64_field_diff(
            &mut diffs,
            &format!("transactions[{}].amount", index),
            g.amount,
            e.amount,
        );
        push_f64_field_diff(
            &mut diffs,
            &format!("transactions[{}].balance", index),
            g.balance,
            e.balance,
        );
    }

    diffs
}

pub fn matches(generated: &StatementData, expected: &StatementData) -> Result<(), String> {
    let diffs = diff(generated, expected);
    if diffs.is_empty() {
        Ok(())
    } else {
        Err(diffs.join("\n"))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Spec {
    pub statement_data: StatementData,
    pub text_items: Vec<TextItem>,
}

impl Spec {
    pub fn new(config_db: &ConfigDB, text_items: Vec<TextItem>) -> Result<Self, String> {
        let sd = text_items_to_statement_data(config_db, &text_items)?;
        Ok(Spec {
            statement_data: sd,
            text_items,
        })
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    pub fn validate(&self, config_db: &ConfigDB) -> Result<(), String> {
        let generated = Spec::new(config_db, self.text_items.clone())?;
        matches(&generated.statement_data, &self.statement_data).map_err(|summary| {
            format!(
                "StatementData generated from spec TextItems does not match StatementData in spec:\n{}",
                summary
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::Transaction;

    fn statement(amount: f64, balance: f64) -> StatementData {
        let mut sd = StatementData::new();
        sd.transactions.push(Transaction::new(
            1_700_000_000_000,
            1,
            "Coffee".to_string(),
            amount,
            balance,
            "ACC-123".to_string(),
        ));
        sd
    }

    #[test]
    fn compares_statement_data_objects() {
        let first = statement(4.5, 104.0);
        let second = statement(4.5, 104.0);
        let mut third = statement(4.5, 104.0);
        third.transactions[0].description = "Tea".to_string();
        assert!(matches(&first, &second).is_ok());
        assert!(matches(&first, &third).is_err());
    }

    #[test]
    fn compares_statement_data_with_float_tolerance() {
        let first = statement(4.5, 104.0);
        assert!(matches(&first, &statement(4.509, 104.009)).is_ok());
        assert!(matches(&first, &statement(4.51, 104.01)).is_err());
    }

    #[test]
    fn returns_diff_summary_for_mismatch() {
        let first = statement(4.5, 104.0);
        let mut second = statement(4.5, 104.02);
        second.transactions[0].account_number = "OTHER".to_string();
        let mismatch = matches(&first, &second).unwrap_err();
        assert!(mismatch.contains("account_number mismatch"));
        assert!(mismatch.contains("balance mismatch"));
    }
}

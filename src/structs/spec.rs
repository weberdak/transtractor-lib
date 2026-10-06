use crate::configs::db::ConfigDB;
use crate::parsers::flows::text_items_to_statement_data::text_items_to_account_data;
use crate::structs::AccountData;
use crate::structs::text_item::TextItem;
use serde::{Deserialize, Serialize};
use serde_json;

const FLOAT_MATCH_TOLERANCE: f64 = 0.01;

fn option_f64_matches(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(l), Some(r)) => (l - r).abs() < FLOAT_MATCH_TOLERANCE,
        (None, None) => true,
        _ => false,
    }
}

fn push_option_field_diff<T: std::fmt::Debug + PartialEq>(
    diffs: &mut Vec<String>,
    field_name: &str,
    generated: Option<T>,
    expected: Option<T>,
) {
    if generated != expected {
        diffs.push(format!(
            "{} mismatch: generated {:?}, expected {:?}",
            field_name, generated, expected
        ));
    }
}

fn push_option_f64_field_diff(
    diffs: &mut Vec<String>,
    field_name: &str,
    generated: Option<f64>,
    expected: Option<f64>,
) {
    match (generated, expected) {
        (Some(g), Some(e)) => {
            if !option_f64_matches(Some(g), Some(e)) {
                let delta = (g - e).abs();
                diffs.push(format!(
                    "{} mismatch: generated {:.6}, expected {:.6}, |delta| {:.6} >= {:.6}",
                    field_name, g, e, delta, FLOAT_MATCH_TOLERANCE
                ));
            }
        }
        (None, None) => {}
        _ => {
            diffs.push(format!(
                "{} mismatch: generated {:?}, expected {:?}",
                field_name, generated, expected
            ));
        }
    }
}

pub fn diff(generated: &AccountData, expected: &AccountData) -> Vec<String> {
    let mut diffs = Vec::new();

    push_option_field_diff(
        &mut diffs,
        "key",
        generated.key.clone(),
        expected.key.clone(),
    );
    push_option_field_diff(
        &mut diffs,
        "account_number",
        generated.account_number.clone(),
        expected.account_number.clone(),
    );
    push_option_field_diff(
        &mut diffs,
        "start_date",
        generated.start_date,
        expected.start_date,
    );
    push_option_field_diff(
        &mut diffs,
        "start_date_year",
        generated.start_date_year,
        expected.start_date_year,
    );
    push_option_f64_field_diff(
        &mut diffs,
        "opening_balance",
        generated.opening_balance,
        expected.opening_balance,
    );
    push_option_f64_field_diff(
        &mut diffs,
        "closing_balance",
        generated.closing_balance,
        expected.closing_balance,
    );

    if generated.proto_transactions.len() != expected.proto_transactions.len() {
        diffs.push(format!(
            "proto_transactions length mismatch: generated {}, expected {}",
            generated.proto_transactions.len(),
            expected.proto_transactions.len()
        ));
    }

    for (index, (g, e)) in generated
        .proto_transactions
        .iter()
        .zip(expected.proto_transactions.iter())
        .enumerate()
    {
        if g.date != e.date {
            diffs.push(format!(
                "proto_transactions[{}].date mismatch: generated {:?}, expected {:?}",
                index, g.date, e.date
            ));
        }
        if g.index != e.index {
            diffs.push(format!(
                "proto_transactions[{}].index mismatch: generated {}, expected {}",
                index, g.index, e.index
            ));
        }
        if g.description != e.description {
            diffs.push(format!(
                "proto_transactions[{}].description mismatch: generated {:?}, expected {:?}",
                index, g.description, e.description
            ));
        }
        push_option_f64_field_diff(
            &mut diffs,
            &format!("proto_transactions[{}].amount", index),
            g.amount,
            e.amount,
        );
        push_option_f64_field_diff(
            &mut diffs,
            &format!("proto_transactions[{}].balance", index),
            g.balance,
            e.balance,
        );
    }

    if generated.errors != expected.errors {
        diffs.push(format!(
            "errors mismatch: generated {:?}, expected {:?}",
            generated.errors, expected.errors
        ));
    }

    diffs
}

pub fn matches(generated: &AccountData, expected: &AccountData) -> Result<(), String> {
    let diffs = diff(generated, expected);
    if diffs.is_empty() {
        Ok(())
    } else {
        Err(diffs.join("\n"))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Spec {
    pub statement_data: AccountData,
    pub text_items: Vec<TextItem>,
}

impl Spec {
    pub fn new(config_db: &ConfigDB, text_items: Vec<TextItem>) -> Result<Self, String> {
        let sd = text_items_to_account_data(config_db, &text_items)?;
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
    use crate::structs::ProtoTransaction;

    #[test]
    fn compares_statement_data_objects() {
        let mut first = AccountData::new();
        first.set_key("statement-1".to_string());
        first.set_account_number("ACC-123".to_string());
        first.add_proto_transaction(ProtoTransaction {
            date: Some(1_700_000_000_000i64),
            index: 1,
            description: "Coffee".to_string(),
            amount: Some(4.5),
            balance: Some(104.0),
        });

        let mut second = AccountData::new();
        second.set_key("statement-1".to_string());
        second.set_account_number("ACC-123".to_string());
        second.add_proto_transaction(ProtoTransaction {
            date: Some(1_700_000_000_000i64),
            index: 1,
            description: "Coffee".to_string(),
            amount: Some(4.5),
            balance: Some(104.0),
        });

        let mut third = AccountData::new();
        third.set_key("statement-2".to_string());

        assert!(matches(&first, &second).is_ok());
        assert!(matches(&first, &third).is_err());
    }

    #[test]
    fn compares_statement_data_with_float_tolerance() {
        let mut first = AccountData::new();
        first.set_opening_balance(100.0);
        first.set_closing_balance(120.0);
        first.add_proto_transaction(ProtoTransaction {
            date: Some(1_700_000_000_000i64),
            index: 1,
            description: "Coffee".to_string(),
            amount: Some(4.5),
            balance: Some(104.0),
        });

        let mut within_tolerance = AccountData::new();
        within_tolerance.set_opening_balance(100.009);
        within_tolerance.set_closing_balance(119.991);
        within_tolerance.add_proto_transaction(ProtoTransaction {
            date: Some(1_700_000_000_000i64),
            index: 1,
            description: "Coffee".to_string(),
            amount: Some(4.509),
            balance: Some(104.009),
        });

        let mut at_tolerance = AccountData::new();
        at_tolerance.set_opening_balance(100.01);
        at_tolerance.set_closing_balance(120.01);
        at_tolerance.add_proto_transaction(ProtoTransaction {
            date: Some(1_700_000_000_000i64),
            index: 1,
            description: "Coffee".to_string(),
            amount: Some(4.51),
            balance: Some(104.01),
        });

        assert!(matches(&first, &within_tolerance).is_ok());
        assert!(matches(&first, &at_tolerance).is_err());
    }

    #[test]
    fn returns_diff_summary_for_mismatch() {
        let mut first = AccountData::new();
        first.set_key("statement-1".to_string());
        first.set_opening_balance(100.0);

        let mut second = AccountData::new();
        second.set_key("statement-2".to_string());
        second.set_opening_balance(100.02);

        let mismatch = matches(&first, &second).unwrap_err();
        assert!(mismatch.contains("key mismatch"));
        assert!(mismatch.contains("opening_balance mismatch"));
    }
}

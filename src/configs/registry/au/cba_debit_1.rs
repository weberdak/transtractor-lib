use crate::structs::StatementConfig;
use regex::Regex;

pub const KEY: &str = "au__cba__debit__1";

pub fn get_config() -> StatementConfig {
    StatementConfig {
        key: KEY.to_string(),
        bank_name: "Commonwealth Bank of Australia".to_string(),
        account_type: "Savings".to_string(),
        account_terms: vec!["CommBank".to_string(), "Account Number".to_string()],
        account_examples: vec![
            "Streamline".to_string(),
            "Smart Access".to_string(),
            "GoalSaver".to_string(),
            "Everyday Offset".to_string(),
        ],

        account_number_terms: vec!["Account Number".to_string()],
        account_number_trigger_count: 1,
        account_number_patterns: vec![Regex::new(r"\b\d+\s\d+\s\d+\b").unwrap()],
        account_number_alignment: "y1".to_string(),
        account_number_alignment_tol: 5,

        opening_balance_terms: vec!["OPENING BALANCE".to_string()],
        opening_balance_trigger_count: 1,
        opening_balance_formats: vec!["format3".to_string(), "format5".to_string()],
        opening_balance_alignment: "y1".to_string(),
        opening_balance_alignment_tol: 5,
        opening_balance_invert: false,

        closing_balance_terms: vec!["Closing balance".to_string(), "Closing Balance".to_string()],
        closing_balance_trigger_count: 1,
        closing_balance_formats: vec!["format3".to_string(), "format5".to_string()],
        closing_balance_alignment: "y1".to_string(),
        closing_balance_alignment_tol: 5,
        closing_balance_invert: false,
        closing_balance_set_from_last_transaction: false,

        start_date_terms: vec!["Statement Period".to_string()],
        start_date_trigger_count: 1,
        start_date_formats: vec!["format2".to_string()],
        start_date_alignment: "y1".to_string(),
        start_date_alignment_tol: 20,

        transaction_terms: vec!["Date Transaction Debit".to_string()],
        transaction_trigger_count: 1,
        transaction_terms_stop: vec![
            "Opening balance".to_string(),
            "Important Information:".to_string(),
        ],
        transaction_stop_trigger_count: 1,
        transaction_formats: vec![vec![
            "date".to_string(),
            "description".to_string(),
            "amount".to_string(),
            "balance".to_string(),
        ]],
        transaction_start_date_required: true,
        transaction_alignment_tol: 20,

        transaction_date_formats: vec!["format1".to_string()],
        transaction_date_headers: vec!["Date".to_string()],
        transaction_date_alignment: "x1".to_string(),
        transaction_date_x1_range: [0, 10000],
        transaction_date_x2_range: [0, 10000],

        transaction_description_headers: vec!["Transaction Details".to_string()],
        transaction_description_alignment: "x1".to_string(),
        transaction_description_x1_range: [0, 10000],
        transaction_description_x2_range: [0, 10000],
        transaction_description_exclude: vec![
            Regex::new(r" -$").unwrap(),
            Regex::new(r" \$$").unwrap(),
            Regex::new(r" Statement \d+ \(Page \d+ of \d+\).*$").unwrap(),
        ],

        transaction_amount_formats: vec!["format1".to_string(), "format2".to_string()],
        transaction_amount_headers: vec!["Credit".to_string()],
        transaction_amount_alignment: "x2".to_string(),
        transaction_amount_x1_range: [0, 10000],
        transaction_amount_x2_range: [0, 10000],
        transaction_amount_invert_headers: vec!["Debit".to_string()],
        transaction_amount_invert_alignment: "x2".to_string(),
        transaction_amount_invert_x1_range: [0, 10000],
        transaction_amount_invert_x2_range: [0, 10000],
        transaction_amount_invert: false,

        transaction_balance_formats: vec!["format2".to_string(), "format3".to_string()],
        transaction_balance_headers: vec!["Balance".to_string()],
        transaction_balance_alignment: "x2".to_string(),
        transaction_balance_x1_range: [0, 10000],
        transaction_balance_x2_range: [0, 10000],
        transaction_balance_invert: false,
        transaction_balance_ignore: false,
    }
}

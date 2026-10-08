use crate::structs::StatementConfig;
use regex::Regex;

pub const KEY: &str = "us__cof__360_combo__1";

pub fn get_config() -> StatementConfig {
    StatementConfig {
        key: KEY.to_string(),
        bank_name: "Capital One".to_string(),
        account_type: "Mixed".to_string(),
        account_terms: vec![
            "Capital One 360®".to_string(),
            "TOTAL ENDING BALANCE".to_string(),
        ],
        account_examples: vec![
            "360 Checking".to_string(),
            "360 Performance Savings".to_string(),
        ],

        account_number_terms: vec![
            "360 Checking -".to_string(),
            "360 Performance Savings -".to_string(),
        ],
        account_number_trigger_count: 1,
        account_number_patterns: vec![Regex::new(r"\b\d+\b").unwrap()],
        account_number_alignment: "y1".to_string(),
        account_number_alignment_tol: 5,

        opening_balance_terms: vec!["All Accounts".to_string()],
        opening_balance_trigger_count: 1,
        opening_balance_formats: vec!["format2".to_string()],
        opening_balance_alignment: "y1".to_string(),
        opening_balance_alignment_tol: 5,
        opening_balance_invert: false,

        closing_balance_terms: vec![],
        closing_balance_trigger_count: 0,
        closing_balance_formats: vec!["format2".to_string()],
        closing_balance_alignment: "y1".to_string(),
        closing_balance_alignment_tol: 5,
        closing_balance_invert: false,
        closing_balance_set_from_last_transaction: false,

        start_date_terms: vec!["STATEMENT PERIOD".to_string()],
        start_date_trigger_count: 1,
        start_date_formats: vec!["format15".to_string()],
        start_date_alignment: "x1".to_string(),
        start_date_alignment_tol: 5,

        transaction_terms: vec!["DATE DESCRIPTION".to_string()],
        transaction_trigger_count: 1,
        transaction_terms_stop: vec!["Note: The last".to_string()],
        transaction_stop_trigger_count: 1,
        transaction_formats: vec![vec![
            "date".to_string(),
            "description".to_string(),
            "amount".to_string(),
            "balance".to_string(),
        ]],
        transaction_start_date_required: true,
        transaction_alignment_tol: 30,

        transaction_date_formats: vec!["format10".to_string()],
        transaction_date_headers: vec!["DATE".to_string()],
        transaction_date_alignment: "x1".to_string(),
        transaction_date_x1_range: [0, 10000],
        transaction_date_x2_range: [0, 10000],

        transaction_description_headers: vec!["DESCRIPTION".to_string()],
        transaction_description_alignment: "x1".to_string(),
        transaction_description_x1_range: [0, 10000],
        transaction_description_x2_range: [0, 10000],
        transaction_description_exclude: vec![Regex::new(r"\b(?:Credit|Debit)\b").unwrap()],

        transaction_amount_formats: vec!["format6".to_string()],
        transaction_amount_headers: vec!["AMOUNT".to_string()],
        transaction_amount_alignment: "x2".to_string(),
        transaction_amount_x1_range: [0, 10000],
        transaction_amount_x2_range: [0, 10000],
        transaction_amount_invert_headers: vec![],
        transaction_amount_invert_alignment: "x1".to_string(),
        transaction_amount_invert_x1_range: [0, 10000],
        transaction_amount_invert_x2_range: [0, 10000],
        transaction_amount_invert: false,

        transaction_balance_formats: vec!["format2".to_string()],
        transaction_balance_headers: vec!["BALANCE".to_string()],
        transaction_balance_alignment: "x2".to_string(),
        transaction_balance_x1_range: [0, 10000],
        transaction_balance_x2_range: [0, 10000],
        transaction_balance_invert: false,
        transaction_balance_ignore: true,
    }
}

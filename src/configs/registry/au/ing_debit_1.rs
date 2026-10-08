use crate::structs::StatementConfig;
use regex::Regex;

pub const KEY: &str = "au__ing__debit__1";

pub fn get_config() -> StatementConfig {
    StatementConfig {
        key: "au__ing__debit__1".to_string(),
        bank_name: "ING Bank Australia".to_string(),
        account_type: "Savings".to_string(),
        account_terms: vec!["Money in".to_string(), "ING Bank (Australia)".to_string()],
        account_examples: vec![
            "Savings Maximiser".to_string(),
            "Orange Everyday".to_string(),
        ],

        account_number_terms: vec![
            "Savings Maximiser number:".to_string(),
            "Orange Everyday number:".to_string(),
        ],
        account_number_trigger_count: 1,
        account_number_patterns: vec![Regex::new(r"\d+").unwrap()],
        account_number_alignment: "y1".to_string(),
        account_number_alignment_tol: 5,

        opening_balance_terms: vec!["Opening balance".to_string()],
        opening_balance_trigger_count: 1,
        opening_balance_formats: vec!["format2".to_string()],
        opening_balance_alignment: "x1".to_string(),
        opening_balance_alignment_tol: 5,
        opening_balance_invert: false,

        closing_balance_terms: vec!["Closing balance".to_string()],
        closing_balance_trigger_count: 1,
        closing_balance_formats: vec!["format2".to_string()],
        closing_balance_alignment: "x1".to_string(),
        closing_balance_alignment_tol: 5,
        closing_balance_invert: false,
        closing_balance_set_from_last_transaction: false,

        start_date_terms: vec!["Statement from:".to_string()],
        start_date_trigger_count: 1,
        start_date_formats: vec!["format4".to_string()],
        start_date_alignment: "y1".to_string(),
        start_date_alignment_tol: 5,

        transaction_terms: vec!["Transactions".to_string()],
        transaction_trigger_count: 1,
        transaction_terms_stop: vec![
            "Interest rate at".to_string(),
            "Please check all".to_string(),
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

        transaction_date_formats: vec!["format4".to_string()],
        transaction_date_headers: vec!["Date".to_string()],
        transaction_date_alignment: "x1".to_string(),
        transaction_date_x1_range: [0, 10000],
        transaction_date_x2_range: [0, 10000],

        transaction_description_headers: vec!["Details".to_string()],
        transaction_description_alignment: "x1".to_string(),
        transaction_description_x1_range: [0, 10000],
        transaction_description_x2_range: [0, 10000],
        transaction_description_exclude: vec![],

        transaction_amount_formats: vec!["format1".to_string()],
        transaction_amount_headers: vec!["Money out $".to_string()],
        transaction_amount_alignment: "x1".to_string(),
        transaction_amount_x1_range: [0, 10000],
        transaction_amount_x2_range: [0, 10000],
        transaction_amount_invert_headers: vec!["Money in $".to_string()],
        transaction_amount_invert_alignment: "x1".to_string(),
        transaction_amount_invert_x1_range: [0, 10000],
        transaction_amount_invert_x2_range: [0, 10000],
        transaction_amount_invert: false,

        transaction_balance_formats: vec!["format1".to_string()],
        transaction_balance_headers: vec!["Balance $".to_string()],
        transaction_balance_alignment: "x1".to_string(),
        transaction_balance_x1_range: [0, 10000],
        transaction_balance_x2_range: [0, 10000],
        transaction_balance_invert: false,
        transaction_balance_ignore: false,
    }
}

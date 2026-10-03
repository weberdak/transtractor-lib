use crate::structs::StatementConfig;
use regex::Regex;

pub const KEY: &str = "au__wbc__debit__1";

pub fn get_config() -> StatementConfig {
    StatementConfig {
        key: "au__wbc__debit__1".to_string(),
        bank_name: "Westpac Banking Corporation Australia".to_string(),
        account_type: "Savings".to_string(),
        account_terms: vec![
            "Westpac Banking Corporation".to_string(),
            "Account Number".to_string(),
        ],
        account_examples: vec!["Choice".to_string(), "Life".to_string()],

        account_number_terms: vec!["Account Number".to_string()],
        account_number_trigger_count: 1,
        account_number_patterns: vec![Regex::new(r"\d+").unwrap()],
        account_number_alignment: "x1".to_string(),
        account_number_alignment_tol: 5,

        opening_balance_terms: vec!["Opening Balance".to_string()],
        opening_balance_trigger_count: 1,
        opening_balance_formats: vec!["format6".to_string()],
        opening_balance_alignment: "y1".to_string(),
        opening_balance_alignment_tol: 5,
        opening_balance_invert: false,

        closing_balance_terms: vec!["Closing Balance".to_string()],
        closing_balance_trigger_count: 1,
        closing_balance_formats: vec!["format6".to_string()],
        closing_balance_alignment: "y1".to_string(),
        closing_balance_alignment_tol: 5,
        closing_balance_invert: false,

        start_date_terms: vec!["Statement Period".to_string()],
        start_date_trigger_count: 1,
        start_date_formats: vec!["format2".to_string()],
        start_date_alignment: "x1".to_string(),
        start_date_alignment_tol: 5,

        transaction_terms: vec!["TRANSACTIONS".to_string()],
        transaction_trigger_count: 1,
        transaction_terms_stop: vec![
            "CLOSING BALANCE".to_string(),
            "MORE INFORMATION".to_string(),
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

        transaction_date_formats: vec!["format5".to_string()],
        transaction_date_headers: vec!["DATE".to_string()],
        transaction_date_alignment: "x1".to_string(),
        transaction_date_x1_range: [0, 10000],
        transaction_date_x2_range: [0, 10000],

        transaction_description_headers: vec!["TRANSACTION DESCRIPTION".to_string()],
        transaction_description_alignment: "x1".to_string(),
        transaction_description_x1_range: [0, 10000],
        transaction_description_x2_range: [0, 10000],
        transaction_description_exclude: vec![
            Regex::new(r" Statement No\. \d+ Page \d+ of \d+.*$").unwrap(),
        ],

        transaction_amount_formats: vec!["format1".to_string()],
        transaction_amount_headers: vec!["CREDIT".to_string()],
        transaction_amount_alignment: "x2".to_string(),
        transaction_amount_x1_range: [0, 10000],
        transaction_amount_x2_range: [0, 10000],
        transaction_amount_invert_headers: vec!["DEBIT".to_string()],
        transaction_amount_invert_alignment: "x2".to_string(),
        transaction_amount_invert_x1_range: [0, 10000],
        transaction_amount_invert_x2_range: [0, 10000],
        transaction_amount_invert: false,

        transaction_balance_formats: vec!["format1".to_string()],
        transaction_balance_headers: vec!["BALANCE".to_string()],
        transaction_balance_alignment: "x2".to_string(),
        transaction_balance_x1_range: [0, 10000],
        transaction_balance_x2_range: [0, 10000],
        transaction_balance_invert: false,
        transaction_balance_ignore: false,
    }
}

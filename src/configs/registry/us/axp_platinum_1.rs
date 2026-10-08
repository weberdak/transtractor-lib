use crate::structs::StatementConfig;
use regex::Regex;

pub const KEY: &str = "us__axp__platinum__1";

pub fn get_config() -> StatementConfig {
    StatementConfig {
        key: KEY.to_string(),
        bank_name: "American Express".to_string(),
        account_type: "Credit Card".to_string(),
        account_terms: vec!["americanexpress.com".to_string(), "Platinum".to_string()],
        account_examples: vec!["Platinum Card".to_string()],

        account_number_terms: vec!["Account Ending".to_string()],
        account_number_trigger_count: 1,
        account_number_patterns: vec![Regex::new(r"\b\d-\d+\b").unwrap()],
        account_number_alignment: "x1".to_string(),
        account_number_alignment_tol: 5,

        opening_balance_terms: vec!["Previous Balance".to_string()],
        opening_balance_trigger_count: 3,
        opening_balance_formats: vec!["format2".to_string()],
        opening_balance_alignment: "y1".to_string(),
        opening_balance_alignment_tol: 5,
        opening_balance_invert: true,

        closing_balance_terms: vec!["New Balance".to_string()],
        closing_balance_trigger_count: 4,
        closing_balance_formats: vec!["format2".to_string()],
        closing_balance_alignment: "y1".to_string(),
        closing_balance_alignment_tol: 5,
        closing_balance_invert: true,
        closing_balance_set_from_last_transaction: false,

        start_date_terms: vec![],
        start_date_trigger_count: 1,
        start_date_formats: vec![],
        start_date_alignment: "y1".to_string(),
        start_date_alignment_tol: 5,

        transaction_terms: vec!["Total Payments and Credits".to_string()],
        transaction_trigger_count: 1,
        transaction_terms_stop: vec!["Total Fees for this Period".to_string()],
        transaction_stop_trigger_count: 1,
        transaction_formats: vec![vec![
            "date".to_string(),
            "description".to_string(),
            "amount".to_string(),
        ]],
        transaction_start_date_required: false,
        transaction_alignment_tol: 20,

        transaction_date_formats: vec!["format9".to_string(), "format14".to_string()],
        transaction_date_headers: vec!["Detail".to_string()],
        transaction_date_alignment: "x1".to_string(),
        transaction_date_x1_range: [0, 10000],
        transaction_date_x2_range: [0, 10000],

        transaction_description_headers: vec!["*Indicates posting date".to_string()],
        transaction_description_alignment: "x1".to_string(),
        transaction_description_x1_range: [0, 10000],
        transaction_description_x2_range: [0, 10000],
        transaction_description_exclude: vec![],

        transaction_amount_formats: vec!["format2".to_string(), "format7".to_string()],
        transaction_amount_headers: vec!["Amount".to_string()],
        transaction_amount_alignment: "x2".to_string(),
        transaction_amount_x1_range: [0, 10000],
        transaction_amount_x2_range: [0, 10000],
        transaction_amount_invert_headers: vec![],
        transaction_amount_invert_alignment: "x1".to_string(),
        transaction_amount_invert_x1_range: [0, 10000],
        transaction_amount_invert_x2_range: [0, 10000],
        transaction_amount_invert: true,

        transaction_balance_formats: vec![],
        transaction_balance_headers: vec![],
        transaction_balance_alignment: "x1".to_string(),
        transaction_balance_x1_range: [0, 10000],
        transaction_balance_x2_range: [0, 10000],
        transaction_balance_invert: false,
        transaction_balance_ignore: false,
    }
}

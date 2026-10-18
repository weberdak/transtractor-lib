use crate::structs::StatementConfig;
use regex::Regex;

pub const KEY: &str = "au__def__personal_loan__1";

pub fn get_config() -> StatementConfig {
    StatementConfig {
        key: KEY.to_string(),
        bank_name: "Defence Bank".to_string(),
        account_type: "Loan".to_string(),
        account_terms: vec!["Defence Bank".to_string(), "PERSONAL LOAN".to_string()],
        account_examples: vec!["Personal Loan".to_string()],

        account_number_terms: vec!["- PERSONAL LOAN".to_string()],
        account_number_trigger_count: 1,
        account_number_patterns: vec![Regex::new(r"^\d{4,}$").unwrap()],
        account_number_alignment: "y1".to_string(),
        account_number_alignment_tol: 5,
        account_number_terms_after: true,

        opening_balance_terms: vec!["OPENING BALANCE ........".to_string()],
        opening_balance_trigger_count: 1,
        opening_balance_formats: vec!["format1".to_string()],
        opening_balance_alignment: "y1".to_string(),
        opening_balance_alignment_tol: 5,
        opening_balance_invert: false,
        opening_balance_inside_table: true,

        closing_balance_terms: vec!["CURRENT BALANCE ........".to_string()],
        closing_balance_trigger_count: 1,
        closing_balance_formats: vec!["format1".to_string()],
        closing_balance_alignment: "y1".to_string(),
        closing_balance_alignment_tol: 5,
        closing_balance_invert: false,
        closing_balance_set_from_last_transaction: false,
        closing_balance_inside_table: true,

        start_date_terms: vec!["Period Begins".to_string()],
        start_date_trigger_count: 1,
        start_date_formats: vec!["format16".to_string()],
        start_date_alignment: "y1".to_string(),
        start_date_alignment_tol: 5,

        transaction_terms: vec!["- PERSONAL LOAN".to_string()],
        transaction_trigger_count: 1,
        transaction_terms_stop: vec!["CURRENT BALANCE ........".to_string()],
        transaction_stop_trigger_count: 1,
        transaction_formats: vec![vec![
            "date".to_string(),
            "description".to_string(),
            "amount".to_string(),
            "balance".to_string(),
        ]],
        transaction_start_date_required: true,
        transaction_alignment_tol: 10,

        transaction_date_formats: vec!["format1".to_string()],
        transaction_date_headers: vec![],
        transaction_date_alignment: "x1".to_string(),
        transaction_date_x1_range: [65, 85],
        transaction_date_x2_range: [0, 10000],

        transaction_description_headers: vec![],
        transaction_description_alignment: "x1".to_string(),
        transaction_description_x1_range: [100, 120],
        transaction_description_x2_range: [0, 10000],
        transaction_description_exclude: vec![],

        transaction_amount_formats: vec!["format1".to_string()],
        transaction_amount_headers: vec![],
        transaction_amount_alignment: "x2".to_string(),
        transaction_amount_x1_range: [0, 10000],
        transaction_amount_x2_range: [290, 365],
        transaction_amount_invert_headers: vec![],
        transaction_amount_invert_alignment: "x2".to_string(),
        transaction_amount_invert_x1_range: [0, 10000],
        transaction_amount_invert_x2_range: [345, 365],
        transaction_amount_invert: false,

        transaction_balance_formats: vec!["format1".to_string()],
        transaction_balance_headers: vec![],
        transaction_balance_alignment: "x2".to_string(),
        transaction_balance_x1_range: [0, 10000],
        transaction_balance_x2_range: [400, 420],
        transaction_balance_invert: false,
        transaction_balance_ignore: false,
    }
}

use crate::structs::StatementConfig;
use regex::Regex;

pub const KEY: &str = "au__def__credit_card__1";

pub fn get_config() -> StatementConfig {
    StatementConfig {
        key: KEY.to_string(),
        bank_name: "Defence Bank".to_string(),
        account_type: "Credit Card".to_string(),
        account_terms: vec!["Credit Limit".to_string(), "Defence Bank".to_string()],
        account_examples: vec![
            "Defence Bank Foundation Credit Card".to_string(),
            "Low Start Credit Card".to_string(),
        ],

        account_number_terms: vec![
            "DEFBANK FOUNDATION CREDIT CARD".to_string(),
            "LOW START CREDIT CARD".to_string(),
        ],
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
        opening_balance_inside_table: false,

        closing_balance_terms: vec!["CURRENT BALANCE ......".to_string()],
        closing_balance_trigger_count: 1,
        closing_balance_formats: vec!["format1".to_string()],
        closing_balance_alignment: "y1".to_string(),
        closing_balance_alignment_tol: 5,
        closing_balance_invert: false,
        closing_balance_set_from_last_transaction: false,
        closing_balance_inside_table: false,

        start_date_terms: vec![],
        start_date_trigger_count: 0,
        start_date_formats: vec!["format16".to_string()],
        start_date_alignment: "y1".to_string(),
        start_date_alignment_tol: 5,

        transaction_terms: vec![
            "DEFBANK FOUNDATION CREDIT CARD".to_string(),
            "LOW START CREDIT CARD".to_string(),
        ],
        transaction_trigger_count: 1,
        transaction_terms_stop: vec!["CURRENT BALANCE ......".to_string()],
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
        transaction_date_x1_range: [100, 120],
        transaction_date_x2_range: [0, 10000],

        transaction_description_headers: vec![],
        transaction_description_alignment: "x1".to_string(),
        transaction_description_x1_range: [175, 200],
        transaction_description_x2_range: [0, 10000],
        transaction_description_exclude: vec![],

        transaction_amount_formats: vec!["format1".to_string()],
        transaction_amount_headers: vec![],
        transaction_amount_alignment: "x2".to_string(),
        transaction_amount_x1_range: [0, 10000],
        transaction_amount_x2_range: [380, 500],
        transaction_amount_invert_headers: vec![],
        transaction_amount_invert_alignment: "x2".to_string(),
        transaction_amount_invert_x1_range: [0, 10000],
        transaction_amount_invert_x2_range: [380, 400],
        transaction_amount_invert: false,

        transaction_balance_formats: vec!["format1".to_string()],
        transaction_balance_headers: vec![],
        transaction_balance_alignment: "x2".to_string(),
        transaction_balance_x1_range: [0, 10000],
        transaction_balance_x2_range: [540, 560],
        transaction_balance_invert: false,
        transaction_balance_ignore: false,
    }
}

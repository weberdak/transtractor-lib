use crate::structs::StatementConfig;
use regex::Regex;

pub const KEY: &str = "au__nab__classic_banking__1";

pub fn get_config() -> StatementConfig {
    StatementConfig {
        key: KEY.to_string(),
        bank_name: "National Australia Bank".to_string(),
        account_type: "Savings".to_string(),
        account_terms: vec!["NAB".to_string(), "Classic Banking".to_string()],
        account_examples: vec!["Classic Banking".to_string()],

        account_number_terms: vec!["Account number".to_string()],
        account_number_trigger_count: 1,
        account_number_patterns: vec![Regex::new(r"\b\d+-\d+-\d+\b").unwrap()],
        account_number_alignment: "y1".to_string(),
        account_number_alignment_tol: 5,

        opening_balance_terms: vec!["Opening balance".to_string(), "Open i n g".to_string()],
        opening_balance_trigger_count: 1,
        opening_balance_formats: vec!["format2".to_string(), "format3".to_string()],
        opening_balance_alignment: "y1".to_string(),
        opening_balance_alignment_tol: 5,
        opening_balance_invert: false,

        closing_balance_terms: vec!["Closing balance".to_string(), "C l osi ng".to_string()],
        closing_balance_trigger_count: 1,
        closing_balance_formats: vec!["format2".to_string(), "format3".to_string()],
        closing_balance_alignment: "y1".to_string(),
        closing_balance_alignment_tol: 5,
        closing_balance_invert: false,

        start_date_terms: vec!["Statement starts".to_string()],
        start_date_trigger_count: 1,
        start_date_formats: vec!["format2".to_string()],
        start_date_alignment: "y1".to_string(),
        start_date_alignment_tol: 5,

        transaction_terms: vec![
            "Transaction Details".to_string(),
            "T rans a ctio n".to_string(),
        ],
        transaction_trigger_count: 1,
        transaction_terms_stop: vec![],
        transaction_stop_trigger_count: 1,
        transaction_formats: vec![
            vec![
                "date".to_string(),
                "description".to_string(),
                "amount".to_string(),
                "balance".to_string(),
            ],
            vec![
                "date".to_string(),
                "description".to_string(),
                "amount".to_string(),
            ],
            vec![
                "description".to_string(),
                "amount".to_string(),
                "balance".to_string(),
            ],
            vec!["description".to_string(), "amount".to_string()],
        ],
        transaction_start_date_required: false,
        transaction_alignment_tol: 40, // Very relaxed to handle messy alignment

        transaction_date_formats: vec!["format2".to_string()],
        transaction_date_headers: vec!["Date".to_string(), "Da t e".to_string()],
        transaction_date_alignment: "x1".to_string(),
        transaction_date_x1_range: [0, 10000],
        transaction_date_x2_range: [0, 10000],

        transaction_description_headers: vec![
            "Particulars".to_string(),
            "Pa rt i c u lars".to_string(),
        ],
        transaction_description_alignment: "x1".to_string(),
        transaction_description_x1_range: [0, 10000],
        transaction_description_x2_range: [0, 10000],
        transaction_description_exclude: vec![
            // Remove dot leaders separating description from amount
            Regex::new(r"\ \.\ ").unwrap(),
            Regex::new(r"\.\.").unwrap(),
            // Lines "Carried forward" and "Brought forward" appear at the top and
            // bottom of the table on every page. Flushing them empties the description
            // field and invalidates the transaction so it is not included in the output.
            Regex::new(r"Carried forward").unwrap(),
            Regex::new(r"Brought forward").unwrap(),
            // Remove table header and footer content scooped into the description field
            Regex::new(r"Particulars").unwrap(),
            Regex::new(r"\.$").unwrap(),
            Regex::new(r"  NAB Classic Banking.*?Debits ").unwrap(),
            Regex::new(r"  National Australia Bank.*?Debits ").unwrap(),
            Regex::new(r"C arri e d.*?Bro u gh t f o rwa rd ").unwrap(),
        ],

        transaction_amount_formats: vec!["format1".to_string()],
        transaction_amount_headers: vec!["Credits".to_string(), "C redi ts".to_string()],
        transaction_amount_alignment: "x2".to_string(),
        transaction_amount_x1_range: [0, 10000],
        transaction_amount_x2_range: [0, 10000],
        transaction_amount_invert_headers: vec!["Debits".to_string(), "De b i t s".to_string()],
        transaction_amount_invert_alignment: "x2".to_string(),
        transaction_amount_invert_x1_range: [0, 10000],
        transaction_amount_invert_x2_range: [0, 10000],
        transaction_amount_invert: false,

        transaction_balance_formats: vec!["format1".to_string(), "format4".to_string()],
        transaction_balance_headers: vec!["Balance".to_string()],
        transaction_balance_alignment: "x2".to_string(),
        transaction_balance_x1_range: [0, 10000],
        transaction_balance_x2_range: [0, 10000],
        transaction_balance_invert: false,
        transaction_balance_ignore: false,
    }
}

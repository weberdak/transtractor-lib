use regex::Regex;

/// Configuration for parsing a bank statement layout.
#[derive(Debug, Clone)]
pub struct StatementConfig {
    // BANK & ACCOUNT DETAILS
    /// Unique key identifying this layout configuration.
    ///  2-letter region Code, bank acronym or short name, account type.
    ///  (e.g., AU__CBA__Debit)
    pub key: String,
    /// Full name of the bank (e.g., Commonwealth Bank of Australia)
    pub bank_name: String,
    /// Account type label (e.g., "Debit", "Credit Card")
    pub account_type: String,
    /// A set of terms on the statement that can uniquely identify the layout type.
    pub account_terms: Vec<String>,
    /// Account types that should work with this layout (e.g., "Streamline", "Everyday Offset")
    pub account_examples: Vec<String>,

    // ACCOUNT NUMBER READ PARAMS
    /// Array of terms to identify the account number line (e.g., "Account Number", "Acct No")
    pub account_number_terms: Vec<String>,
    /// Number of times a term must be encountered before the parser is considered primed
    pub account_number_trigger_count: usize,
    /// Array of regex patterns to extract the account number
    pub account_number_patterns: Vec<Regex>,
    /// Alignment of the account number relative to the term ("x1", "x2", "y1", "y2", "")
    pub account_number_alignment: String,
    /// Tolerance for alignment matching of account number
    pub account_number_alignment_tol: i32,

    // OPENING BALANCE READ PARAMS
    /// Array of terms to identify the opening balance line (e.g., "Opening Balance", "Previous Balance")
    pub opening_balance_terms: Vec<String>,
    /// Number of times a term must be encountered before the parser is considered primed
    pub opening_balance_trigger_count: usize,
    /// Array of accepted formats to parse the opening balance amount
    pub opening_balance_formats: Vec<String>,
    /// Alignment of the opening balance relative to the term ("x1", "x2", "y1", "y2", "")
    pub opening_balance_alignment: String,
    /// Tolerance for alignment matching of opening balance
    pub opening_balance_alignment_tol: i32,
    /// Invert the sign of the opening balance amount
    pub opening_balance_invert: bool,

    // CLOSING BALANCE READ PARAMS
    /// Array of terms to identify the closing balance line (e.g., "Closing Balance", "New Balance")
    pub closing_balance_terms: Vec<String>,
    /// Number of times a term must be encountered before the parser is considered primed
    pub closing_balance_trigger_count: usize,
    /// Array of accepted formats to parse the closing balance amount
    pub closing_balance_formats: Vec<String>,
    /// Alignment of the closing balance relative to the term ("x1", "x2", "y1", "y2", "")
    pub closing_balance_alignment: String,
    /// Tolerance for alignment matching of closing balance
    pub closing_balance_alignment_tol: i32,
    /// Invert the sign of the closing balance amount
    pub closing_balance_invert: bool,

    // START DATE READ PARAMS
    /// Array of terms to identify the statement start date line (e.g., "Statement Period", "From")
    pub start_date_terms: Vec<String>,
    /// Number of times a term must be encountered before the parser is considered primed
    pub start_date_trigger_count: usize,
    /// Array of accepted formats to parse the statement start date
    pub start_date_formats: Vec<String>,
    /// Alignment of the start date relative to the term ("x1", "x2", "y1", "y2", "")
    pub start_date_alignment: String,
    /// Tolerance for alignment matching of start date
    pub start_date_alignment_tol: i32,

    // GENERAL TRANSACTION READ PARAMS
    /// Array of terms that can indicate start, or nearing the start of transaction table
    /// (e.g., "Transactions").
    pub transaction_terms: Vec<String>,
    /// Number of times a term must be encountered before the parser is considered primed
    pub transaction_trigger_count: usize,
    /// Array of terms that indicate the end, or close after the end of the transaction table.
    pub transaction_terms_stop: Vec<String>,
    /// Number of times a stop term must be encountered before the parser is considered primed
    pub transaction_stop_trigger_count: usize,
    /// Fields expected for a complete transaction line, in order.
    /// E.g., [["date", "description", "amount"], ["description", "amount"]]
    /// Is a common format for credit card statements where the date is only specified
    /// on the first transaction of each day.
    pub transaction_formats: Vec<Vec<String>>,
    /// Parsing transaction requires the start date value to have been read
    /// so that the year can be inferred for each transaction date.
    pub transaction_start_date_required: bool,
    /// Tolerance for X alignment mismatch between value and header
    pub transaction_alignment_tol: i32,

    // TRANSACTION DATE READ PARAMS
    /// Array of accepted formats to parse the transaction date
    pub transaction_date_formats: Vec<String>,
    /// Headers that identify the transaction date column
    pub transaction_date_headers: Vec<String>,
    /// Alignment of the transaction date column ("x1, "x2")
    pub transaction_date_alignment: String,
    /// Inclusive x1 coordinate range for transaction dates
    pub transaction_date_x1_range: [i32; 2],
    /// Inclusive x2 coordinate range for transaction dates
    pub transaction_date_x2_range: [i32; 2],

    // TRANSACTION DESCRIPTION READ PARAMS
    /// Headers that identify the transaction description column
    pub transaction_description_headers: Vec<String>,
    /// Alignment of the transaction description column ("x1, "x2")
    pub transaction_description_alignment: String,
    /// Inclusive x1 coordinate range for transaction descriptions
    pub transaction_description_x1_range: [i32; 2],
    /// Inclusive x2 coordinate range for transaction descriptions
    pub transaction_description_x2_range: [i32; 2],
    /// Regex patterns to exclude from being considered as part of the description.
    /// E.g., [/\.\./g] to exclude "......." patterns.
    pub transaction_description_exclude: Vec<Regex>,

    // TRANSACTION AMOUNT READ PARAMS
    /// Array of accepted formats to parse the transaction amount
    pub transaction_amount_formats: Vec<String>,
    /// Headers that identify the transaction amount column
    pub transaction_amount_headers: Vec<String>,
    /// Alignment of the transaction amount column ("x1, "x2")
    pub transaction_amount_alignment: String,
    /// Inclusive x1 coordinate range for transaction amounts
    pub transaction_amount_x1_range: [i32; 2],
    /// Inclusive x2 coordinate range for transaction amounts
    pub transaction_amount_x2_range: [i32; 2],
    /// Headers that identify when to invert the transaction amount sign
    pub transaction_amount_invert_headers: Vec<String>,
    /// Alignment of the transaction amount invert column ("x1, "x2")
    pub transaction_amount_invert_alignment: String,
    /// Inclusive x1 coordinate range for amounts in the inverted column
    pub transaction_amount_invert_x1_range: [i32; 2],
    /// Inclusive x2 coordinate range for amounts in the inverted column
    pub transaction_amount_invert_x2_range: [i32; 2],
    /// Invert the sign of all transaction amounts. Often needed for credit card statements.
    pub transaction_amount_invert: bool,

    // TRANSACTION BALANCE READ PARAMS
    /// Array of accepted formats to parse the transaction balance amount
    pub transaction_balance_formats: Vec<String>,
    /// Headers that identify the transaction balance column
    pub transaction_balance_headers: Vec<String>,
    /// Alignment of the transaction balance column ("x1, "x2")
    pub transaction_balance_alignment: String,
    /// Inclusive x1 coordinate range for transaction balances
    pub transaction_balance_x1_range: [i32; 2],
    /// Inclusive x2 coordinate range for transaction balances
    pub transaction_balance_x2_range: [i32; 2],
    /// Invert the sign of all transaction balance amounts.
    pub transaction_balance_invert: bool,
    /// Ignore statement balance column if present. Balances will be filled from transaction amounts.
    pub transaction_balance_ignore: bool,
}

impl Default for StatementConfig {
    fn default() -> Self {
        StatementConfig {
            key: "Generic Statement".to_string(),
            bank_name: "Generic Bank".to_string(),
            account_type: "Generic Account".to_string(),
            account_terms: vec![],
            account_examples: vec![],

            account_number_terms: vec![],
            account_number_trigger_count: 1,
            account_number_patterns: vec![],
            account_number_alignment: "y1".to_string(),
            account_number_alignment_tol: 5,

            opening_balance_terms: vec![],
            opening_balance_trigger_count: 1,
            opening_balance_formats: vec![],
            opening_balance_alignment: "y1".to_string(),
            opening_balance_alignment_tol: 5,
            opening_balance_invert: false,

            closing_balance_terms: vec![],
            closing_balance_trigger_count: 1,
            closing_balance_formats: vec![],
            closing_balance_alignment: "y1".to_string(),
            closing_balance_alignment_tol: 5,
            closing_balance_invert: false,

            start_date_terms: vec![],
            start_date_trigger_count: 1,
            start_date_formats: vec![],
            start_date_alignment: "y1".to_string(),
            start_date_alignment_tol: 5,

            transaction_terms: vec![],
            transaction_trigger_count: 1,
            transaction_terms_stop: vec![],
            transaction_stop_trigger_count: 1,
            transaction_formats: vec![],
            transaction_start_date_required: false,
            transaction_alignment_tol: 10,

            transaction_date_formats: vec![],
            transaction_date_headers: vec![],
            transaction_date_alignment: "x1".to_string(),
            transaction_date_x1_range: [0, 10000],
            transaction_date_x2_range: [0, 10000],

            transaction_description_headers: vec![],
            transaction_description_alignment: "x1".to_string(),
            transaction_description_x1_range: [0, 10000],
            transaction_description_x2_range: [0, 10000],
            transaction_description_exclude: vec![],

            transaction_amount_formats: vec![],
            transaction_amount_headers: vec![],
            transaction_amount_alignment: "x1".to_string(),
            transaction_amount_x1_range: [0, 10000],
            transaction_amount_x2_range: [0, 10000],
            transaction_amount_invert_headers: vec![],
            transaction_amount_invert_alignment: "x1".to_string(),
            transaction_amount_invert_x1_range: [0, 10000],
            transaction_amount_invert_x2_range: [0, 10000],
            transaction_amount_invert: false,

            transaction_balance_formats: vec![],
            transaction_balance_headers: vec![],
            transaction_balance_alignment: "x1".to_string(),
            transaction_balance_x1_range: [0, 10000],
            transaction_balance_x2_range: [0, 10000],
            transaction_balance_invert: false,
            transaction_balance_ignore: false,
        }
    }
}

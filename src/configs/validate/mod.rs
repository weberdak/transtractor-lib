use crate::structs::StatementConfig;

pub mod account_examples;
pub mod account_number_alignment;
pub mod account_number_alignment_tol;
pub mod account_number_patterns;
pub mod account_number_terms;
pub mod account_terms;
pub mod account_type;
pub mod bank_name;
pub mod closing_balance_alignment;
pub mod closing_balance_alignment_tol;
pub mod closing_balance_formats;
pub mod closing_balance_terms;
pub mod key;
pub mod opening_balance_alignment;
pub mod opening_balance_alignment_tol;
pub mod opening_balance_formats;
pub mod opening_balance_terms;
pub mod start_date_alignment;
pub mod start_date_alignment_tol;
pub mod start_date_formats;
pub mod start_date_terms;
pub mod transaction_alignment_tol;
pub mod transaction_amount_alignment;
pub mod transaction_amount_formats;
pub mod transaction_amount_headers;
pub mod transaction_amount_invert_alignment;
pub mod transaction_amount_invert_headers;
pub mod transaction_balance_alignment;
pub mod transaction_balance_formats;
pub mod transaction_balance_headers;
pub mod transaction_date_alignment;
pub mod transaction_date_formats;
pub mod transaction_date_headers;
pub mod transaction_description_alignment;
pub mod transaction_description_headers;
pub mod transaction_formats;
pub mod transaction_terms;
pub mod transaction_terms_stop;
pub mod utils;

/// Validate the entire StatementConfig
pub fn validate_config(config: &StatementConfig) -> Result<(), String> {
    key::key(&config.key)?;
    bank_name::bank_name(&config.bank_name)?;
    account_type::account_type(&config.account_type)?;
    account_terms::account_terms(&config.account_terms)?;
    account_examples::account_examples(&config.account_examples)?;
    account_number_terms::account_number_terms(&config.account_number_terms)?;
    account_number_patterns::account_number_patterns(&config.account_number_patterns)?;
    account_number_alignment::account_number_alignment(&config.account_number_alignment)?;
    account_number_alignment_tol::account_number_alignment_tol(
        config.account_number_alignment_tol,
    )?;
    opening_balance_terms::opening_balance_terms(&config.opening_balance_terms)?;
    opening_balance_formats::opening_balance_formats(&config.opening_balance_formats)?;
    opening_balance_alignment::opening_balance_alignment(&config.opening_balance_alignment)?;
    opening_balance_alignment_tol::opening_balance_alignment_tol(
        config.opening_balance_alignment_tol,
    )?;
    // opening_balance_invert is a bool, no validation needed
    closing_balance_terms::closing_balance_terms(&config.closing_balance_terms)?;
    closing_balance_formats::closing_balance_formats(&config.closing_balance_formats)?;
    closing_balance_alignment::closing_balance_alignment(&config.closing_balance_alignment)?;
    closing_balance_alignment_tol::closing_balance_alignment_tol(
        config.closing_balance_alignment_tol,
    )?;
    // closing_balance_invert and closing_balance_set_from_last_transaction are bools, no validation needed
    start_date_terms::start_date_terms(&config.start_date_terms)?;
    start_date_formats::start_date_formats(&config.start_date_formats)?;
    start_date_alignment::start_date_alignment(&config.start_date_alignment)?;
    start_date_alignment_tol::start_date_alignment_tol(config.start_date_alignment_tol)?;
    transaction_terms::transaction_terms(&config.transaction_terms)?;
    transaction_terms_stop::transaction_terms_stop(&config.transaction_terms_stop)?;
    transaction_formats::transaction_formats(&config.transaction_formats)?;
    // transaction_start_date_required is a bool, no validation needed
    transaction_alignment_tol::transaction_alignment_tol(config.transaction_alignment_tol)?;
    transaction_date_formats::transaction_date_formats(&config.transaction_date_formats)?;
    transaction_date_headers::transaction_date_headers(&config.transaction_date_headers)?;
    transaction_date_alignment::transaction_date_alignment(&config.transaction_date_alignment)?;
    transaction_description_headers::transaction_description_headers(
        &config.transaction_description_headers,
    )?;
    transaction_description_alignment::transaction_description_alignment(
        &config.transaction_description_alignment,
    )?;
    // transaction_description_exclude is not validated
    transaction_amount_formats::transaction_amount_formats(&config.transaction_amount_formats)?;
    transaction_amount_headers::transaction_amount_headers(&config.transaction_amount_headers)?;
    transaction_amount_alignment::transaction_amount_alignment(
        &config.transaction_amount_alignment,
    )?;
    transaction_amount_invert_headers::transaction_amount_invert_headers(
        &config.transaction_amount_invert_headers,
    )?;
    transaction_amount_invert_alignment::transaction_amount_invert_alignment(
        &config.transaction_amount_invert_alignment,
    )?;
    // transaction_amount_invert is a bool, no validation needed
    transaction_balance_formats::transaction_balance_formats(&config.transaction_balance_formats)?;
    transaction_balance_headers::transaction_balance_headers(&config.transaction_balance_headers)?;
    transaction_balance_alignment::transaction_balance_alignment(
        &config.transaction_balance_alignment,
    )?;
    // transaction_balance_invert is a bool, no validation needed
    Ok(())
}

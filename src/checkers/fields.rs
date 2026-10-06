use crate::structs::AccountData;

/// Check if required fields are set in the statement data and log errors for missing fields.
///
/// This function checks for the presence of critical statement fields and logs errors
/// when they are missing. Unlike check_balances which panics on missing transaction data,
/// this function gracefully logs issues for missing statement-level fields.
///
/// Checks performed:
/// - Opening balance is set
/// - Closing balance is set
///
/// The function adds error messages to the statement data's error collection for any
/// missing required fields.
pub fn check_fields(sd: &mut AccountData) {
    let mut missing_fields = Vec::new();

    // Check for account number
    if sd.account_number.is_none() {
        missing_fields.push("account number");
    }

    // Check for opening balance
    if sd.opening_balance.is_none() {
        missing_fields.push("opening balance");
    }

    // Check for closing balance
    if sd.closing_balance.is_none() {
        missing_fields.push("closing balance");
    }

    // Log and add errors for missing fields
    if !missing_fields.is_empty() {
        let error_message = format!("Missing required fields: {}", missing_fields.join(", "));
        sd.add_error(error_message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_fields_all_missing() {
        let mut sd = AccountData::new();

        check_fields(&mut sd);
        assert_eq!(sd.errors.len(), 1);
        assert!(
            sd.errors[0].contains(
                "Missing required fields: account number, opening balance, closing balance"
            )
        );
    }

    #[test]
    fn test_check_fields_missing_opening_balance() {
        let mut sd = AccountData::new();
        sd.set_closing_balance(1000.0);

        check_fields(&mut sd);
        assert_eq!(sd.errors.len(), 1);
        assert!(sd.errors[0].contains("Missing required fields: account number, opening balance"));
    }

    #[test]
    fn test_check_fields_missing_closing_balance() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);

        check_fields(&mut sd);
        assert_eq!(sd.errors.len(), 1);
        assert!(sd.errors[0].contains("Missing required fields: account number, closing balance"));
    }

    #[test]
    fn test_check_fields_all_present() {
        let mut sd = AccountData::new();
        sd.set_account_number("1234 5678 9012".to_string());
        sd.set_opening_balance(1000.0);
        sd.set_closing_balance(900.0);

        check_fields(&mut sd);
        assert_eq!(sd.errors.len(), 0);
    }

    #[test]
    fn test_check_fields_with_zero_balances() {
        let mut sd = AccountData::new();
        sd.set_account_number("1234 5678 9012".to_string());
        sd.set_opening_balance(0.0);
        sd.set_closing_balance(0.0);

        check_fields(&mut sd);
        assert_eq!(sd.errors.len(), 0);
    }

    #[test]
    fn test_check_fields_with_negative_balances() {
        let mut sd = AccountData::new();
        sd.set_account_number("1234 5678 9012".to_string());
        sd.set_opening_balance(-500.0);
        sd.set_closing_balance(-200.0);

        check_fields(&mut sd);
        assert_eq!(sd.errors.len(), 0);
    }

    #[test]
    fn test_check_fields_does_not_duplicate_errors() {
        let mut sd = AccountData::new();

        // Call check_fields twice
        check_fields(&mut sd);
        check_fields(&mut sd);

        // Should have 2 error entries (one from each call)
        assert_eq!(sd.errors.len(), 2);
        assert!(
            sd.errors[0].contains(
                "Missing required fields: account number, opening balance, closing balance"
            )
        );
        assert!(
            sd.errors[1].contains(
                "Missing required fields: account number, opening balance, closing balance"
            )
        );
    }
}

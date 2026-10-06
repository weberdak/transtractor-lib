use crate::structs::AccountData;

/// Check if the statement balances are consistent by calculating running balances.
///
/// This function starts with the opening balance and successively adds each transaction amount
/// to calculate a running balance. It then verifies:
/// - Each calculated running balance matches the transaction's stated balance
/// - The final calculated balance matches the statement's closing balance
///
/// # Panics
///
/// Panics if required data is missing (this should not happen during runtime):
/// - Any transaction is missing an amount or balance
///
pub fn check_balances(sd: &mut AccountData) {
    // Log error and return if either balance is missing
    if sd.opening_balance.is_none() || sd.closing_balance.is_none() {
        sd.add_error("Cannot check balances if opening or closing balance is missing".to_string());
        return;
    }

    // Start with opening balance
    let opening_balance = sd.opening_balance.unwrap();
    let closing_balance = sd.closing_balance.unwrap();
    let mut running_balance = opening_balance;
    let mut errors = Vec::new();

    // Round to 2 decimal places to avoid floating point precision issues
    running_balance = (running_balance * 100.0).round() / 100.0;

    // Check each transaction
    for (index, transaction) in sd.proto_transactions.iter().enumerate() {
        // Panic if transaction data is missing
        let transaction_amount = transaction.amount.unwrap_or_else(|| {
            panic!("Transaction {index} must have an amount set before calling check_balances")
        });

        let transaction_balance = transaction.balance.unwrap_or_else(|| {
            panic!("Transaction {index} must have a balance set before calling check_balances")
        });

        // Add transaction amount to running balance
        running_balance += transaction_amount;

        // Round to 2 decimal places to avoid floating point precision issues
        running_balance = (running_balance * 100.0).round() / 100.0;
        let transaction_balance = (transaction_balance * 100.0).round() / 100.0;

        // Check if calculated balance matches transaction balance
        if (running_balance - transaction_balance).abs() > 0.01 {
            let difference = (running_balance - transaction_balance).abs();
            errors.push(format!(
                "Transaction {} balance mismatch. Calculated: {:.2}, Stated: {:.2}, Difference: {:.2}",
                index + 1, running_balance, transaction_balance, difference
            ));
        }
    }

    // Add all transaction balance errors
    for error in errors {
        sd.add_error(error);
    }

    // Check final balance against closing balance
    let closing_balance = (closing_balance * 100.0).round() / 100.0;
    if (running_balance - closing_balance).abs() > 0.01 {
        let difference = (running_balance - closing_balance).abs();
        sd.add_error(format!(
            "Final balance mismatch. Calculated: {:.2}, Stated: {:.2}, Difference: {:.2}",
            running_balance, closing_balance, difference
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::ProtoTransaction;

    /// Helper function to create a transaction with amount and balance
    fn create_transaction(amount: f64, balance: f64) -> ProtoTransaction {
        let mut tx = ProtoTransaction::new();
        tx.set_amount(amount);
        tx.set_balance(balance);
        tx
    }

    #[test]
    fn test_check_balances_missing_opening_balance() {
        let mut sd = AccountData::new();
        sd.set_closing_balance(1000.0);

        check_balances(&mut sd);

        assert_eq!(sd.errors.len(), 1);
        assert!(
            sd.errors[0].contains("Cannot check balances if opening or closing balance is missing")
        );
    }

    #[test]
    fn test_check_balances_missing_closing_balance() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);

        check_balances(&mut sd);

        assert_eq!(sd.errors.len(), 1);
        assert!(
            sd.errors[0].contains("Cannot check balances if opening or closing balance is missing")
        );
    }

    #[test]
    fn test_check_balances_missing_both_balances() {
        let mut sd = AccountData::new();

        check_balances(&mut sd);

        assert_eq!(sd.errors.len(), 1);
        assert!(
            sd.errors[0].contains("Cannot check balances if opening or closing balance is missing")
        );
    }

    #[test]
    #[should_panic(expected = "Transaction 0 must have an amount set")]
    fn test_check_balances_panic_missing_transaction_amount() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);
        sd.set_closing_balance(900.0);

        let mut tx = ProtoTransaction::new();
        // No amount set
        tx.set_balance(900.0);
        sd.add_proto_transaction(tx);

        check_balances(&mut sd);
    }

    #[test]
    #[should_panic(expected = "Transaction 0 must have a balance set")]
    fn test_check_balances_panic_missing_transaction_balance() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);
        sd.set_closing_balance(900.0);

        let mut tx = ProtoTransaction::new();
        tx.set_amount(-100.0);
        // No balance set
        sd.add_proto_transaction(tx);

        check_balances(&mut sd);
    }

    #[test]
    fn test_check_balances_no_transactions_balanced() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);
        sd.set_closing_balance(1000.0);
        // No transactions

        check_balances(&mut sd);

        assert_eq!(sd.errors.len(), 0);
    }

    #[test]
    fn test_check_balances_no_transactions_unbalanced() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);
        sd.set_closing_balance(900.0);
        // No transactions

        check_balances(&mut sd);

        assert_eq!(sd.errors.len(), 1);
        assert!(sd.errors[0].contains("Final balance mismatch"));
        assert!(sd.errors[0].contains("Calculated: 1000.00, Stated: 900.00, Difference: 100.00"));
    }

    #[test]
    fn test_check_balances_single_transaction_balanced() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);
        sd.set_closing_balance(900.0);

        sd.add_proto_transaction(create_transaction(-100.0, 900.0));

        check_balances(&mut sd);

        assert_eq!(sd.errors.len(), 0);
    }

    #[test]
    fn test_check_balances_single_transaction_balance_mismatch() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);
        sd.set_closing_balance(900.0);

        sd.add_proto_transaction(create_transaction(-100.0, 850.0)); // Should be 900

        check_balances(&mut sd);

        assert_eq!(sd.errors.len(), 1);
        // Only one error: transaction balance mismatch
        assert!(sd.errors[0].contains("Transaction 1 balance mismatch"));
        assert!(sd.errors[0].contains("Calculated: 900.00, Stated: 850.00, Difference: 50.00"));
    }

    #[test]
    fn test_check_balances_multiple_transactions_balanced() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);
        sd.set_closing_balance(925.0);

        // Transaction 1: 1000 - 50 = 950
        sd.add_proto_transaction(create_transaction(-50.0, 950.0));

        // Transaction 2: 950 + 100 = 1050
        sd.add_proto_transaction(create_transaction(100.0, 1050.0));

        // Transaction 3: 1050 - 125 = 925
        sd.add_proto_transaction(create_transaction(-125.0, 925.0));

        check_balances(&mut sd);

        assert_eq!(sd.errors.len(), 0);
    }

    #[test]
    fn test_check_balances_multiple_transactions_middle_error() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);
        sd.set_closing_balance(925.0);

        // Transaction 1: 1000 - 50 = 950 (correct)
        sd.add_proto_transaction(create_transaction(-50.0, 950.0));

        // Transaction 2: 950 + 100 = 1050, but transaction says 1000 (error)
        sd.add_proto_transaction(create_transaction(100.0, 1000.0));

        // Transaction 3: Running balance is 1050, so 1050 - 125 = 925 (correct for running balance)
        sd.add_proto_transaction(create_transaction(-125.0, 925.0));

        check_balances(&mut sd);

        assert_eq!(sd.errors.len(), 1);
        // Only error: transaction 2 balance mismatch
        assert!(sd.errors[0].contains("Transaction 2 balance mismatch"));
        assert!(sd.errors[0].contains("Calculated: 1050.00, Stated: 1000.00"));
    }

    #[test]
    fn test_check_balances_final_balance_mismatch_only() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);
        sd.set_closing_balance(800.0); // Wrong final balance

        sd.add_proto_transaction(create_transaction(-100.0, 900.0)); // Correct transaction

        check_balances(&mut sd);

        assert_eq!(sd.errors.len(), 1);
        assert!(sd.errors[0].contains("Final balance mismatch"));
        assert!(sd.errors[0].contains("Calculated: 900.00, Stated: 800.00, Difference: 100.00"));
    }

    #[test]
    fn test_check_balances_floating_point_precision() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);
        sd.set_closing_balance(999.90);

        // Use a transaction amount that could cause floating point precision issues
        sd.add_proto_transaction(create_transaction(-0.1, 999.899999)); // Should round to 999.90

        check_balances(&mut sd);

        assert_eq!(sd.errors.len(), 0);
    }

    #[test]
    fn test_check_balances_negative_amounts() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(-500.0);
        sd.set_closing_balance(-700.0);

        sd.add_proto_transaction(create_transaction(-200.0, -700.0));

        check_balances(&mut sd);

        assert_eq!(sd.errors.len(), 0);
    }

    #[test]
    fn test_check_balances_positive_amounts() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(100.0);
        sd.set_closing_balance(400.0);

        sd.add_proto_transaction(create_transaction(300.0, 400.0));

        check_balances(&mut sd);

        assert_eq!(sd.errors.len(), 0);
    }

    #[test]
    fn test_check_balances_zero_amounts() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);
        sd.set_closing_balance(1000.0);

        sd.add_proto_transaction(create_transaction(0.0, 1000.0));

        check_balances(&mut sd);

        assert_eq!(sd.errors.len(), 0);
    }

    #[test]
    fn test_check_balances_error_messages_contain_transaction_numbers() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);
        sd.set_closing_balance(600.0);

        // Transaction 1: correct
        sd.add_proto_transaction(create_transaction(-100.0, 900.0));

        // Transaction 2: incorrect - should be 800, but stated as 750
        sd.add_proto_transaction(create_transaction(-100.0, 750.0));

        // Transaction 3: incorrect - running balance is 800, so 800 - 100 = 700, but stated as 650
        sd.add_proto_transaction(create_transaction(-100.0, 650.0));

        check_balances(&mut sd);

        // Should have errors for transactions 2 and 3, plus potentially final balance error
        assert!(sd.errors.len() >= 2);
        assert!(sd.errors[0].contains("Transaction 2"));
        assert!(sd.errors[1].contains("Transaction 3"));
    }

    #[test]
    fn test_check_balances_rounding_consistency() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(100.0);
        sd.set_closing_balance(99.67);

        // This should result in exactly 99.67 after rounding
        sd.add_proto_transaction(create_transaction(-0.33, 99.67));

        check_balances(&mut sd);

        assert_eq!(sd.errors.len(), 0);
    }

    #[test]
    fn test_check_balances_large_numbers() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1_000_000.0);
        sd.set_closing_balance(999_999.99);

        sd.add_proto_transaction(create_transaction(-0.01, 999_999.99));

        check_balances(&mut sd);

        assert_eq!(sd.errors.len(), 0);
    }
}

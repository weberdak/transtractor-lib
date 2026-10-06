use crate::structs::AccountData;

/// Fix opening balance if it does not match the first transaction. This
/// usually occurs when the opening balance is unsigned or the first
/// transaction is a debit.
///
/// This function checks if the opening balance plus the first transaction amount
/// equals the first transaction balance. If not, it tries:
/// 1. Reversing the sign of the opening balance
/// 2. Reversing the sign of the first transaction amount (treating it as a debit)
pub fn fix_opening_balance(sd: &mut AccountData) {
    // Return early if no transactions
    if sd.proto_transactions.is_empty() {
        return;
    }

    // Return early if first transaction is missing amount or balance
    let first_transaction = &sd.proto_transactions[0];
    let first_amount = match first_transaction.amount {
        Some(amount) => amount,
        None => return,
    };
    let first_balance = match first_transaction.balance {
        Some(balance) => balance,
        None => return,
    };

    // Return early if no opening balance
    let opening_balance = match sd.opening_balance {
        Some(balance) => balance,
        None => return,
    };

    const TOLERANCE: f64 = 0.01;

    // Opening and first balance agree, no issue
    if (first_balance - (opening_balance + first_amount)).abs() < TOLERANCE {
        return;
    }

    // Try reversing sign of opening balance
    if (first_balance - (-opening_balance + first_amount)).abs() < TOLERANCE {
        sd.set_opening_balance(-opening_balance);
        return;
    }

    // First amount is a debit, reverse sign of first amount
    if (first_balance - (opening_balance - first_amount)).abs() < TOLERANCE {
        sd.proto_transactions[0].set_amount(-first_amount);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::ProtoTransaction;

    #[test]
    fn test_fix_opening_balance_no_transactions() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(100.0);

        // Should not panic with no transactions
        fix_opening_balance(&mut sd);

        // Opening balance should remain unchanged
        assert_eq!(sd.opening_balance, Some(100.0));
    }

    #[test]
    fn test_fix_opening_balance_no_opening_balance() {
        let mut sd = AccountData::new();

        // Add transaction with amount and balance
        let mut tx1 = ProtoTransaction::new();
        tx1.set_amount(50.0);
        tx1.set_balance(150.0);
        sd.add_proto_transaction(tx1);

        // Should return early when no opening balance
        fix_opening_balance(&mut sd);

        // Transaction should remain unchanged
        assert_eq!(sd.proto_transactions[0].amount, Some(50.0));
        assert_eq!(sd.proto_transactions[0].balance, Some(150.0));
    }

    #[test]
    fn test_fix_opening_balance_first_transaction_no_amount() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(100.0);

        // Add transaction without amount
        let mut tx1 = ProtoTransaction::new();
        tx1.set_balance(150.0);
        sd.add_proto_transaction(tx1);

        // Should return early when first transaction has no amount
        fix_opening_balance(&mut sd);

        // Opening balance should remain unchanged
        assert_eq!(sd.opening_balance, Some(100.0));
    }

    #[test]
    fn test_fix_opening_balance_first_transaction_no_balance() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(100.0);

        // Add transaction without balance
        let mut tx1 = ProtoTransaction::new();
        tx1.set_amount(50.0);
        sd.add_proto_transaction(tx1);

        // Should return early when first transaction has no balance
        fix_opening_balance(&mut sd);

        // Opening balance should remain unchanged
        assert_eq!(sd.opening_balance, Some(100.0));
    }

    #[test]
    fn test_fix_opening_balance_already_correct() {
        let mut sd = AccountData::new();
        let opening_balance = 100.0;
        let first_amount = 50.0;
        let first_balance = opening_balance + first_amount; // 150.0

        sd.set_opening_balance(opening_balance);

        let mut tx1 = ProtoTransaction::new();
        tx1.set_amount(first_amount);
        tx1.set_balance(first_balance);
        sd.add_proto_transaction(tx1);

        fix_opening_balance(&mut sd);

        // Nothing should change when balance is already correct
        assert_eq!(sd.opening_balance, Some(opening_balance));
        assert_eq!(sd.proto_transactions[0].amount, Some(first_amount));
        assert_eq!(sd.proto_transactions[0].balance, Some(first_balance));
    }

    #[test]
    fn test_fix_opening_balance_reverse_opening_balance_sign() {
        let mut sd = AccountData::new();
        let incorrect_opening_balance = 100.0; // Should be -100.0
        let first_amount = 50.0;
        let first_balance = 50.0 - 100.0; // -50.0 (correct calculation with -100.0 opening)

        sd.set_opening_balance(incorrect_opening_balance);

        let mut tx1 = ProtoTransaction::new();
        tx1.set_amount(first_amount);
        tx1.set_balance(first_balance);
        sd.add_proto_transaction(tx1);

        fix_opening_balance(&mut sd);

        // Opening balance sign should be reversed
        assert_eq!(sd.opening_balance, Some(-incorrect_opening_balance));
        assert_eq!(sd.proto_transactions[0].amount, Some(first_amount));
        assert_eq!(sd.proto_transactions[0].balance, Some(first_balance));
    }

    #[test]
    fn test_fix_opening_balance_reverse_first_amount_sign() {
        let mut sd = AccountData::new();
        let opening_balance = 100.0;
        let incorrect_first_amount = 50.0; // Should be -50.0 (debit)
        let first_balance = 50.0; // 100.0 - 50.0 = 50.0 (correct with -50.0 amount)

        sd.set_opening_balance(opening_balance);

        let mut tx1 = ProtoTransaction::new();
        tx1.set_amount(incorrect_first_amount);
        tx1.set_balance(first_balance);
        sd.add_proto_transaction(tx1);

        fix_opening_balance(&mut sd);

        // First amount sign should be reversed
        assert_eq!(sd.opening_balance, Some(opening_balance));
        assert_eq!(
            sd.proto_transactions[0].amount,
            Some(-incorrect_first_amount)
        );
        assert_eq!(sd.proto_transactions[0].balance, Some(first_balance));
    }

    #[test]
    fn test_fix_opening_balance_within_tolerance() {
        let mut sd = AccountData::new();
        let opening_balance = 100.0;
        let first_amount = 50.0;
        let first_balance = 150.005; // Very close to correct value (150.0)

        sd.set_opening_balance(opening_balance);

        let mut tx1 = ProtoTransaction::new();
        tx1.set_amount(first_amount);
        tx1.set_balance(first_balance);
        sd.add_proto_transaction(tx1);

        fix_opening_balance(&mut sd);

        // Should be considered correct within tolerance
        assert_eq!(sd.opening_balance, Some(opening_balance));
        assert_eq!(sd.proto_transactions[0].amount, Some(first_amount));
        assert_eq!(sd.proto_transactions[0].balance, Some(first_balance));
    }

    #[test]
    fn test_fix_opening_balance_no_solution_found() {
        let mut sd = AccountData::new();
        let opening_balance = 100.0;
        let first_amount = 50.0;
        let first_balance = 200.0; // No correction can make this work

        sd.set_opening_balance(opening_balance);

        let mut tx1 = ProtoTransaction::new();
        tx1.set_amount(first_amount);
        tx1.set_balance(first_balance);
        sd.add_proto_transaction(tx1);

        fix_opening_balance(&mut sd);

        // Nothing should change when no solution is found
        assert_eq!(sd.opening_balance, Some(opening_balance));
        assert_eq!(sd.proto_transactions[0].amount, Some(first_amount));
        assert_eq!(sd.proto_transactions[0].balance, Some(first_balance));
    }

    #[test]
    fn test_fix_opening_balance_negative_amounts() {
        let mut sd = AccountData::new();
        let opening_balance = -100.0;
        let first_amount = -50.0;
        let first_balance = -150.0; // -100.0 + (-50.0) = -150.0

        sd.set_opening_balance(opening_balance);

        let mut tx1 = ProtoTransaction::new();
        tx1.set_amount(first_amount);
        tx1.set_balance(first_balance);
        sd.add_proto_transaction(tx1);

        fix_opening_balance(&mut sd);

        // Should work correctly with negative values
        assert_eq!(sd.opening_balance, Some(opening_balance));
        assert_eq!(sd.proto_transactions[0].amount, Some(first_amount));
        assert_eq!(sd.proto_transactions[0].balance, Some(first_balance));
    }

    #[test]
    fn test_fix_opening_balance_multiple_transactions_only_checks_first() {
        let mut sd = AccountData::new();
        let opening_balance = 100.0; // Should be -100.0
        let first_amount = 50.0;
        let first_balance = -50.0; // Correct with -100.0 opening balance

        sd.set_opening_balance(opening_balance);

        // First transaction (will be fixed)
        let mut tx1 = ProtoTransaction::new();
        tx1.set_amount(first_amount);
        tx1.set_balance(first_balance);
        sd.add_proto_transaction(tx1);

        // Second transaction (should be ignored)
        let mut tx2 = ProtoTransaction::new();
        tx2.set_amount(25.0);
        tx2.set_balance(-25.0);
        sd.add_proto_transaction(tx2);

        fix_opening_balance(&mut sd);

        // Opening balance should be fixed, only first transaction considered
        assert_eq!(sd.opening_balance, Some(-opening_balance));
        assert_eq!(sd.proto_transactions[0].amount, Some(first_amount));
        assert_eq!(sd.proto_transactions[0].balance, Some(first_balance));
        // Second transaction unchanged
        assert_eq!(sd.proto_transactions[1].amount, Some(25.0));
        assert_eq!(sd.proto_transactions[1].balance, Some(-25.0));
    }
}

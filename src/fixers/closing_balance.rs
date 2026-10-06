use crate::structs::AccountData;

/// Reverse the sign of the closing balance only if it is inconsistent with
/// the sum of the opening balance and all transaction amounts.
///
/// This function calculates the expected closing balance by summing the opening
/// balance with all transaction amounts. If the actual closing balance is closer
/// to the negative of this expected value, it reverses the sign of the closing balance.
pub fn fix_closing_balance(sd: &mut AccountData) {
    // Start with the opening balance, return early if not set
    let mut balance = match sd.opening_balance {
        Some(opening_balance) => opening_balance,
        None => return, // Can't fix closing balance without opening balance
    };

    // Sum all transaction amounts to calculate expected closing balance
    for transaction in &sd.proto_transactions {
        if let Some(amount) = transaction.amount {
            balance += amount;
        }
    }

    // Check if the closing balance should be reversed
    if let Some(closing_balance) = sd.closing_balance {
        // Check if the negative of the calculated balance is closer to the actual closing balance
        // Using a small tolerance (0.01) for floating point comparison
        let diff_with_negative = (-balance - closing_balance).abs();
        let diff_with_positive = (balance - closing_balance).abs();

        // If the negative calculated balance is much closer (within tolerance), reverse the sign
        if diff_with_negative < 0.01 && diff_with_negative < diff_with_positive {
            sd.set_closing_balance(-closing_balance);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::ProtoTransaction;

    #[test]
    fn test_fix_closing_balance_no_opening_balance() {
        let mut sd = AccountData::new();
        sd.set_closing_balance(500.0);

        // Should not panic when opening balance is None
        fix_closing_balance(&mut sd);

        // Closing balance should remain unchanged
        assert_eq!(sd.closing_balance, Some(500.0));
    }

    #[test]
    fn test_fix_closing_balance_no_closing_balance() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);

        // Should not panic when closing balance is None
        fix_closing_balance(&mut sd);

        // Nothing should change
        assert_eq!(sd.closing_balance, None);
    }

    #[test]
    fn test_fix_closing_balance_no_transactions() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);
        sd.set_closing_balance(1000.0);

        fix_closing_balance(&mut sd);

        // With no transactions, closing should equal opening, so no change
        assert_eq!(sd.closing_balance, Some(1000.0));
    }

    #[test]
    fn test_fix_closing_balance_reverses_incorrect_sign() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);
        sd.set_closing_balance(900.0); // Should be -900.0

        // Add transaction: +100, so expected closing = 1000 + 100 = 1100
        // Actual closing is 900, but -1100 = -1100 is not close to 900
        // Let's make it so the closing should be negative
        let mut tx1 = ProtoTransaction::new();
        tx1.set_amount(-2000.0); // 1000 - 2000 = -1000
        tx1.description = "Large withdrawal".to_string();
        sd.add_proto_transaction(tx1);

        // Expected: 1000 + (-2000) = -1000
        // Actual: 900
        // Check: |-(-1000) - 900| = |1000 - 900| = 100 (not < 0.01)
        // Let's set closing to 1000 so that -(-1000) - 1000 = 0
        sd.set_closing_balance(1000.0);

        fix_closing_balance(&mut sd);

        // The closing balance should be reversed to -1000
        assert_eq!(sd.closing_balance, Some(-1000.0));
    }

    #[test]
    fn test_fix_closing_balance_leaves_correct_sign() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);
        sd.set_closing_balance(1100.0);

        // Add transaction: +100, so expected closing = 1000 + 100 = 1100
        let mut tx1 = ProtoTransaction::new();
        tx1.set_amount(100.0);
        tx1.description = "Deposit".to_string();
        sd.add_proto_transaction(tx1);

        fix_closing_balance(&mut sd);

        // The closing balance should remain unchanged
        assert_eq!(sd.closing_balance, Some(1100.0));
    }

    #[test]
    fn test_fix_closing_balance_with_multiple_transactions() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);

        // Add multiple transactions: +100, -50, +25 = +75 total
        // Expected closing: 1000 + 75 = 1075
        let mut tx1 = ProtoTransaction::new();
        tx1.set_amount(100.0);
        tx1.description = "Deposit".to_string();
        sd.add_proto_transaction(tx1);

        let mut tx2 = ProtoTransaction::new();
        tx2.set_amount(-50.0);
        tx2.description = "Withdrawal".to_string();
        sd.add_proto_transaction(tx2);

        let mut tx3 = ProtoTransaction::new();
        tx3.set_amount(25.0);
        tx3.description = "Interest".to_string();
        sd.add_proto_transaction(tx3);

        // Set closing balance to the negative of expected (should be corrected)
        sd.set_closing_balance(-1075.0);

        fix_closing_balance(&mut sd);

        // Should be corrected to positive
        assert_eq!(sd.closing_balance, Some(1075.0));
    }

    #[test]
    fn test_fix_closing_balance_skips_transactions_without_amount() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);
        sd.set_closing_balance(1000.0);

        // Add transaction without amount
        let mut tx1 = ProtoTransaction::new();
        tx1.description = "Incomplete transaction".to_string();
        // amount is None by default
        sd.add_proto_transaction(tx1);

        // Add transaction with amount
        let mut tx2 = ProtoTransaction::new();
        tx2.set_amount(50.0);
        tx2.description = "Complete transaction".to_string();
        sd.add_proto_transaction(tx2);

        fix_closing_balance(&mut sd);

        // Expected: 1000 + 0 + 50 = 1050, actual: 1000
        // Difference is 50, which is > 0.01, so no change
        assert_eq!(sd.closing_balance, Some(1000.0));
    }

    #[test]
    fn test_fix_closing_balance_within_tolerance() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);

        // Add transaction
        let mut tx1 = ProtoTransaction::new();
        tx1.set_amount(100.0);
        tx1.description = "Transaction".to_string();
        sd.add_proto_transaction(tx1);

        // Expected: 1000 + 100 = 1100
        // Set closing to -1100 + small amount (should still trigger correction)
        sd.set_closing_balance(-1099.999);

        fix_closing_balance(&mut sd);

        // Should be corrected to positive
        assert_eq!(sd.closing_balance, Some(1099.999));
    }
}

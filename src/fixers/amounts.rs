use crate::structs::AccountData;

/// Reverse the sign of transaction amounts if the balance is inconsistent
/// with the sum of the previous balance and the amount.
///
/// This function iterates through all proto transactions and checks if the
/// transaction balance is consistent with: previous_balance + transaction_amount.
/// If the balance is more consistent with: previous_balance - transaction_amount,
/// then it reverses the sign of the transaction amount.
pub fn fix_amounts(sd: &mut AccountData) {
    // Start with the opening balance, return early if not set
    let mut balance = match sd.opening_balance {
        Some(opening_balance) => opening_balance,
        None => return, // Can't fix amounts without opening balance
    };

    for transaction in &mut sd.proto_transactions {
        // Skip transactions that don't have both amount and balance
        let (amount, transaction_balance) = match (transaction.amount, transaction.balance) {
            (Some(amt), Some(bal)) => (amt, bal),
            _ => continue,
        };

        // Check if the balance is more consistent with the reversed amount
        // Using a small tolerance (0.01) for floating point comparison
        let expected_balance_with_current_amount = balance + amount;
        let expected_balance_with_reversed_amount = balance - amount;

        let diff_current = (transaction_balance - expected_balance_with_current_amount).abs();
        let diff_reversed = (transaction_balance - expected_balance_with_reversed_amount).abs();

        // If the reversed amount gives a better match, reverse the transaction amount
        if diff_reversed < diff_current && diff_reversed < 0.01 {
            transaction.set_amount(-amount);
        }

        // Update the running balance to the actual transaction balance
        balance = transaction_balance;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::ProtoTransaction;

    #[test]
    fn test_fix_amounts_no_opening_balance() {
        let mut sd = AccountData::new();
        // Should not panic when opening balance is None
        fix_amounts(&mut sd);
        assert_eq!(sd.proto_transactions.len(), 0);
    }

    #[test]
    fn test_fix_amounts_no_transactions() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);
        fix_amounts(&mut sd);
        assert_eq!(sd.proto_transactions.len(), 0);
    }

    #[test]
    fn test_fix_amounts_reverses_incorrect_signs() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);

        // Create a transaction where the amount should be negative
        // Opening: 1000, Amount: +100, Balance: 900
        // This suggests the amount should be -100
        let mut tx1 = ProtoTransaction::new();
        tx1.amount = Some(100.0);
        tx1.balance = Some(900.0);
        tx1.description = "Test transaction".to_string();
        sd.add_proto_transaction(tx1);

        fix_amounts(&mut sd);

        // The amount should now be -100
        assert_eq!(sd.proto_transactions[0].amount, Some(-100.0));
    }

    #[test]
    fn test_fix_amounts_leaves_correct_signs() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);

        // Create a transaction where the amount is already correct
        // Opening: 1000, Amount: +100, Balance: 1100
        let mut tx1 = ProtoTransaction::new();
        tx1.amount = Some(100.0);
        tx1.balance = Some(1100.0);
        tx1.description = "Test transaction".to_string();
        sd.add_proto_transaction(tx1);

        fix_amounts(&mut sd);

        // The amount should remain +100
        assert_eq!(sd.proto_transactions[0].amount, Some(100.0));
    }

    #[test]
    fn test_fix_amounts_skips_incomplete_transactions() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);

        // Transaction with missing amount
        let mut tx1 = ProtoTransaction::new();
        tx1.amount = None;
        tx1.balance = Some(900.0);
        tx1.description = "Test transaction".to_string();
        sd.add_proto_transaction(tx1);

        // Transaction with missing balance
        let mut tx2 = ProtoTransaction::new();
        tx2.amount = Some(100.0);
        tx2.balance = None;
        tx2.description = "Test transaction 2".to_string();
        sd.add_proto_transaction(tx2);

        fix_amounts(&mut sd);

        // Both transactions should remain unchanged
        assert_eq!(sd.proto_transactions[0].amount, None);
        assert_eq!(sd.proto_transactions[1].balance, None);
    }

    #[test]
    fn test_fix_amounts_multiple_transactions() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);

        // First transaction: 1000 + 50 = 1050 (correct)
        let mut tx1 = ProtoTransaction::new();
        tx1.amount = Some(50.0);
        tx1.balance = Some(1050.0);
        tx1.description = "Deposit".to_string();
        sd.add_proto_transaction(tx1);

        // Second transaction: 1050 + 200 = 850 (should be -200)
        let mut tx2 = ProtoTransaction::new();
        tx2.amount = Some(200.0);
        tx2.balance = Some(850.0);
        tx2.description = "Withdrawal".to_string();
        sd.add_proto_transaction(tx2);

        fix_amounts(&mut sd);

        // First transaction should remain +50
        assert_eq!(sd.proto_transactions[0].amount, Some(50.0));
        // Second transaction should become -200
        assert_eq!(sd.proto_transactions[1].amount, Some(-200.0));
    }
}

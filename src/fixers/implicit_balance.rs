use crate::structs::AccountData;

/// Fix transactions with implicit balances. Occurs when the statement does
/// not provide a balance for a transaction, usually for credit card statements.
///
/// This function calculates missing transaction balances by maintaining a running
/// balance starting from the opening balance and adding each transaction amount.
/// For transactions that already have a balance, it uses that balance to continue
/// the calculation for subsequent transactions.
pub fn fix_implicit_balances(sd: &mut AccountData) {
    // Start with the opening balance, return early if not set
    let mut balance = match sd.opening_balance {
        Some(opening_balance) => opening_balance,
        None => return, // Can't fix implicit balances without opening balance
    };

    for transaction in &mut sd.proto_transactions {
        // Skip transactions that don't have an amount
        if let Some(amount) = transaction.amount {
            // If the transaction doesn't have a balance, calculate it
            if let Some(existing_balance) = transaction.balance {
                // If the transaction already has a balance, use it for the next calculation
                balance = existing_balance;
            } else {
                let new_balance = balance + amount;
                transaction.set_balance(new_balance);
                balance = new_balance;
            }
        }
        // If transaction has no amount, the balance remains unchanged for next iteration
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::ProtoTransaction;

    #[test]
    fn test_fix_implicit_balances_no_opening_balance() {
        let mut sd = AccountData::new();

        // Add a transaction with amount but no balance
        let mut tx1 = ProtoTransaction::new();
        tx1.set_amount(100.0);
        tx1.description = "Test transaction".to_string();
        sd.add_proto_transaction(tx1);

        // Should not panic when opening balance is None
        fix_implicit_balances(&mut sd);

        // Transaction balance should remain None
        assert_eq!(sd.proto_transactions[0].balance, None);
    }

    #[test]
    fn test_fix_implicit_balances_no_transactions() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);

        // Should not panic with no transactions
        fix_implicit_balances(&mut sd);

        // Nothing should change
        assert_eq!(sd.proto_transactions.len(), 0);
    }

    #[test]
    fn test_fix_implicit_balances_single_transaction() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);

        // Add transaction with amount but no balance
        let mut tx1 = ProtoTransaction::new();
        tx1.set_amount(50.0);
        tx1.description = "Deposit".to_string();
        sd.add_proto_transaction(tx1);

        fix_implicit_balances(&mut sd);

        // Balance should be calculated: 1000 + 50 = 1050
        assert_eq!(sd.proto_transactions[0].balance, Some(1050.0));
    }

    #[test]
    fn test_fix_implicit_balances_multiple_transactions() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);

        // Add multiple transactions with amounts but no balances
        let mut tx1 = ProtoTransaction::new();
        tx1.set_amount(50.0);
        tx1.description = "Deposit".to_string();
        sd.add_proto_transaction(tx1);

        let mut tx2 = ProtoTransaction::new();
        tx2.set_amount(-30.0);
        tx2.description = "Withdrawal".to_string();
        sd.add_proto_transaction(tx2);

        let mut tx3 = ProtoTransaction::new();
        tx3.set_amount(100.0);
        tx3.description = "Another deposit".to_string();
        sd.add_proto_transaction(tx3);

        fix_implicit_balances(&mut sd);

        // Balances should be calculated sequentially
        assert_eq!(sd.proto_transactions[0].balance, Some(1050.0)); // 1000 + 50
        assert_eq!(sd.proto_transactions[1].balance, Some(1020.0)); // 1050 - 30
        assert_eq!(sd.proto_transactions[2].balance, Some(1120.0)); // 1020 + 100
    }

    #[test]
    fn test_fix_implicit_balances_mixed_existing_and_missing() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);

        // First transaction: has amount, no balance
        let mut tx1 = ProtoTransaction::new();
        tx1.set_amount(50.0);
        tx1.description = "Deposit".to_string();
        sd.add_proto_transaction(tx1);

        // Second transaction: has both amount and balance (should use existing balance)
        let mut tx2 = ProtoTransaction::new();
        tx2.set_amount(-30.0);
        tx2.set_balance(900.0); // Different from calculated balance
        tx2.description = "Withdrawal".to_string();
        sd.add_proto_transaction(tx2);

        // Third transaction: has amount, no balance (should use tx2's balance)
        let mut tx3 = ProtoTransaction::new();
        tx3.set_amount(25.0);
        tx3.description = "Interest".to_string();
        sd.add_proto_transaction(tx3);

        fix_implicit_balances(&mut sd);

        // First transaction should get calculated balance
        assert_eq!(sd.proto_transactions[0].balance, Some(1050.0)); // 1000 + 50

        // Second transaction should keep its existing balance
        assert_eq!(sd.proto_transactions[1].balance, Some(900.0));

        // Third transaction should use second transaction's balance
        assert_eq!(sd.proto_transactions[2].balance, Some(925.0)); // 900 + 25
    }

    #[test]
    fn test_fix_implicit_balances_skips_transactions_without_amount() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);

        // First transaction: has amount and no balance
        let mut tx1 = ProtoTransaction::new();
        tx1.set_amount(50.0);
        tx1.description = "Deposit".to_string();
        sd.add_proto_transaction(tx1);

        // Second transaction: no amount, no balance (should be skipped)
        let mut tx2 = ProtoTransaction::new();
        tx2.description = "Incomplete transaction".to_string();
        sd.add_proto_transaction(tx2);

        // Third transaction: has amount and no balance
        let mut tx3 = ProtoTransaction::new();
        tx3.set_amount(25.0);
        tx3.description = "Another deposit".to_string();
        sd.add_proto_transaction(tx3);

        fix_implicit_balances(&mut sd);

        // First transaction should get calculated balance
        assert_eq!(sd.proto_transactions[0].balance, Some(1050.0)); // 1000 + 50

        // Second transaction should remain None for both amount and balance
        assert_eq!(sd.proto_transactions[1].amount, None);
        assert_eq!(sd.proto_transactions[1].balance, None);

        // Third transaction should use balance from first transaction (skipping second)
        assert_eq!(sd.proto_transactions[2].balance, Some(1075.0)); // 1050 + 25
    }

    #[test]
    fn test_fix_implicit_balances_preserves_existing_balances() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);

        // Transaction with both amount and balance already set
        let mut tx1 = ProtoTransaction::new();
        tx1.set_amount(50.0);
        tx1.set_balance(1200.0); // Different from calculated
        tx1.description = "Transaction with existing balance".to_string();
        sd.add_proto_transaction(tx1);

        fix_implicit_balances(&mut sd);

        // Existing balance should be preserved
        assert_eq!(sd.proto_transactions[0].balance, Some(1200.0));
    }

    #[test]
    fn test_fix_implicit_balances_negative_amounts() {
        let mut sd = AccountData::new();
        sd.set_opening_balance(1000.0);

        // Add transactions with negative amounts
        let mut tx1 = ProtoTransaction::new();
        tx1.set_amount(-100.0);
        tx1.description = "Withdrawal".to_string();
        sd.add_proto_transaction(tx1);

        let mut tx2 = ProtoTransaction::new();
        tx2.set_amount(-50.0);
        tx2.description = "Fee".to_string();
        sd.add_proto_transaction(tx2);

        fix_implicit_balances(&mut sd);

        // Balances should decrease
        assert_eq!(sd.proto_transactions[0].balance, Some(900.0)); // 1000 - 100
        assert_eq!(sd.proto_transactions[1].balance, Some(850.0)); // 900 - 50
    }
}

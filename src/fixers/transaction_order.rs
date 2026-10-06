use crate::structs::AccountData;

/// Reorder proto-transactions by date and then by index.
///
/// This function sorts transactions chronologically by date, and then by index
/// for transactions with the same date. This ensures proper transaction ordering
/// for accurate balance calculations and statement processing.
///
/// The reordering is only performed if NONE of the proto-transactions have a
/// balance set. This is because if balances are already present, the transaction
/// order might be critical for balance consistency and should not be altered.
///
/// Sorting criteria:
/// 1. Primary: by date (oldest first)
/// 2. Secondary: by index (lowest first) for transactions with the same date
///
/// # Panics
///
/// Panics if any transaction does not have a date set. All transactions should
/// have dates before this fixer is called.
pub fn fix_transaction_order(sd: &mut AccountData) {
    // Check if any transaction has a balance set
    // If so, we should not reorder as it might break balance consistency
    let has_any_balance = sd.proto_transactions.iter().any(|tx| tx.balance.is_some());

    if has_any_balance {
        return; // Don't reorder if any transaction has a balance
    }

    // Verify all transactions have dates - panic if not
    for (i, tx) in sd.proto_transactions.iter().enumerate() {
        if tx.date.is_none() {
            panic!(
                "Transaction at index {} does not have a date set. All transactions must have dates before reordering.",
                i
            );
        }
    }

    // Sort by date first, then by index
    sd.proto_transactions.sort_by(|a, b| {
        let date_a = a.date.unwrap(); // Safe to unwrap after verification above
        let date_b = b.date.unwrap(); // Safe to unwrap after verification above

        // Primary sort: by date
        match date_a.cmp(&date_b) {
            std::cmp::Ordering::Equal => {
                // Secondary sort: by index if dates are equal
                a.index.cmp(&b.index)
            }
            other => other,
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::ProtoTransaction;

    #[test]
    fn test_fix_transaction_order_sorts_by_date_and_index() {
        let mut sd = AccountData::new();

        // Create transactions in wrong order
        let mut tx1 = ProtoTransaction::new();
        tx1.date = Some(1000);
        tx1.index = 2;
        tx1.description = "Transaction 1".to_string();

        let mut tx2 = ProtoTransaction::new();
        tx2.date = Some(500);
        tx2.index = 1;
        tx2.description = "Transaction 2".to_string();

        let mut tx3 = ProtoTransaction::new();
        tx3.date = Some(1000);
        tx3.index = 1;
        tx3.description = "Transaction 3".to_string();

        sd.proto_transactions = vec![tx1, tx2, tx3];

        fix_transaction_order(&mut sd);

        // Should be sorted by date first, then by index
        assert_eq!(sd.proto_transactions[0].date, Some(500));
        assert_eq!(sd.proto_transactions[0].index, 1);
        assert_eq!(sd.proto_transactions[0].description, "Transaction 2");

        assert_eq!(sd.proto_transactions[1].date, Some(1000));
        assert_eq!(sd.proto_transactions[1].index, 1);
        assert_eq!(sd.proto_transactions[1].description, "Transaction 3");

        assert_eq!(sd.proto_transactions[2].date, Some(1000));
        assert_eq!(sd.proto_transactions[2].index, 2);
        assert_eq!(sd.proto_transactions[2].description, "Transaction 1");
    }

    #[test]
    #[should_panic(expected = "Transaction at index 1 does not have a date set")]
    fn test_fix_transaction_order_panics_on_none_dates() {
        let mut sd = AccountData::new();

        let mut tx1 = ProtoTransaction::new();
        tx1.date = Some(1000);
        tx1.index = 1;
        tx1.description = "Transaction with date".to_string();

        let mut tx2 = ProtoTransaction::new();
        tx2.date = None; // This should cause a panic
        tx2.index = 2;
        tx2.description = "Transaction without date".to_string();

        sd.proto_transactions = vec![tx1, tx2];

        fix_transaction_order(&mut sd); // Should panic here
    }

    #[test]
    fn test_fix_transaction_order_does_not_reorder_if_balance_present() {
        let mut sd = AccountData::new();

        let mut tx1 = ProtoTransaction::new();
        tx1.date = Some(1000);
        tx1.index = 2;
        tx1.balance = Some(500.0); // Balance is set
        tx1.description = "Transaction 1".to_string();

        let mut tx2 = ProtoTransaction::new();
        tx2.date = Some(500);
        tx2.index = 1;
        tx2.description = "Transaction 2".to_string();

        let original_order = [tx1.clone(), tx2.clone()];
        sd.proto_transactions = vec![tx1, tx2];

        fix_transaction_order(&mut sd);

        // Order should remain unchanged because tx1 has a balance
        assert_eq!(sd.proto_transactions[0].date, original_order[0].date);
        assert_eq!(
            sd.proto_transactions[0].description,
            original_order[0].description
        );

        assert_eq!(sd.proto_transactions[1].date, original_order[1].date);
        assert_eq!(
            sd.proto_transactions[1].description,
            original_order[1].description
        );
    }

    #[test]
    fn test_fix_transaction_order_empty_transactions() {
        let mut sd = AccountData::new();

        fix_transaction_order(&mut sd);

        // Should handle empty transactions without panic
        assert!(sd.proto_transactions.is_empty());
    }

    #[test]
    fn test_fix_transaction_order_single_transaction() {
        let mut sd = AccountData::new();

        let mut tx = ProtoTransaction::new();
        tx.date = Some(1000);
        tx.index = 1;
        tx.description = "Single transaction".to_string();

        sd.proto_transactions = vec![tx.clone()];

        fix_transaction_order(&mut sd);

        // Single transaction should remain unchanged
        assert_eq!(sd.proto_transactions.len(), 1);
        assert_eq!(sd.proto_transactions[0].date, tx.date);
        assert_eq!(sd.proto_transactions[0].description, tx.description);
    }
}

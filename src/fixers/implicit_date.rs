use crate::structs::AccountData;

/// Fix transactions with implicit dates. Occurs when the date is implied
/// by the previous transaction's date.
///
/// This function fills in missing transaction dates by maintaining a running
/// date starting from the statement start date. For transactions that already
/// have a date, it uses that date to continue the sequence for subsequent transactions.
pub fn fix_implicit_dates(sd: &mut AccountData) {
    // Start with the start date, return early if not set
    let mut date = match sd.start_date {
        Some(start_date) => start_date,
        None => return, // Can't fix implicit dates without start date
    };

    for transaction in &mut sd.proto_transactions {
        if let Some(tx_date) = transaction.date {
            // If the transaction already has a date, use it for subsequent transactions
            date = tx_date;
        } else {
            // If the transaction doesn't have a date, use the current date
            transaction.set_date(date);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::ProtoTransaction;

    #[test]
    fn test_fix_implicit_dates_no_start_date() {
        let mut sd = AccountData::new();

        // Add a transaction without date
        let mut tx1 = ProtoTransaction::new();
        tx1.description = "Test transaction".to_string();
        sd.add_proto_transaction(tx1);

        // Should not panic when start date is None
        fix_implicit_dates(&mut sd);

        // Transaction date should remain None
        assert_eq!(sd.proto_transactions[0].date, None);
    }

    #[test]
    fn test_fix_implicit_dates_no_transactions() {
        let mut sd = AccountData::new();
        sd.set_start_date(1609459200000); // 2021-01-01 timestamp

        // Should not panic with no transactions
        fix_implicit_dates(&mut sd);

        // Nothing should change
        assert_eq!(sd.proto_transactions.len(), 0);
    }

    #[test]
    fn test_fix_implicit_dates_single_transaction() {
        let mut sd = AccountData::new();
        let start_date = 1609459200000; // 2021-01-01 timestamp
        sd.set_start_date(start_date);

        // Add transaction without date
        let mut tx1 = ProtoTransaction::new();
        tx1.description = "Transaction without date".to_string();
        sd.add_proto_transaction(tx1);

        fix_implicit_dates(&mut sd);

        // Transaction should get the start date
        assert_eq!(sd.proto_transactions[0].date, Some(start_date));
    }

    #[test]
    fn test_fix_implicit_dates_multiple_transactions_all_missing() {
        let mut sd = AccountData::new();
        let start_date = 1609459200000; // 2021-01-01 timestamp
        sd.set_start_date(start_date);

        // Add multiple transactions without dates
        let mut tx1 = ProtoTransaction::new();
        tx1.description = "Transaction 1".to_string();
        sd.add_proto_transaction(tx1);

        let mut tx2 = ProtoTransaction::new();
        tx2.description = "Transaction 2".to_string();
        sd.add_proto_transaction(tx2);

        let mut tx3 = ProtoTransaction::new();
        tx3.description = "Transaction 3".to_string();
        sd.add_proto_transaction(tx3);

        fix_implicit_dates(&mut sd);

        // All transactions should get the start date
        assert_eq!(sd.proto_transactions[0].date, Some(start_date));
        assert_eq!(sd.proto_transactions[1].date, Some(start_date));
        assert_eq!(sd.proto_transactions[2].date, Some(start_date));
    }

    #[test]
    fn test_fix_implicit_dates_mixed_existing_and_missing() {
        let mut sd = AccountData::new();
        let start_date = 1609459200000; // 2021-01-01 timestamp
        let tx2_date = 1609545600000; // 2021-01-02 timestamp
        let tx4_date = 1609632000000; // 2021-01-03 timestamp

        sd.set_start_date(start_date);

        // First transaction: no date (should get start date)
        let mut tx1 = ProtoTransaction::new();
        tx1.description = "Transaction 1".to_string();
        sd.add_proto_transaction(tx1);

        // Second transaction: has date (should use this date for subsequent)
        let mut tx2 = ProtoTransaction::new();
        tx2.set_date(tx2_date);
        tx2.description = "Transaction 2".to_string();
        sd.add_proto_transaction(tx2);

        // Third transaction: no date (should get tx2's date)
        let mut tx3 = ProtoTransaction::new();
        tx3.description = "Transaction 3".to_string();
        sd.add_proto_transaction(tx3);

        // Fourth transaction: has date (should use this date)
        let mut tx4 = ProtoTransaction::new();
        tx4.set_date(tx4_date);
        tx4.description = "Transaction 4".to_string();
        sd.add_proto_transaction(tx4);

        // Fifth transaction: no date (should get tx4's date)
        let mut tx5 = ProtoTransaction::new();
        tx5.description = "Transaction 5".to_string();
        sd.add_proto_transaction(tx5);

        fix_implicit_dates(&mut sd);

        // Check the results
        assert_eq!(sd.proto_transactions[0].date, Some(start_date)); // tx1 gets start date
        assert_eq!(sd.proto_transactions[1].date, Some(tx2_date)); // tx2 keeps its date
        assert_eq!(sd.proto_transactions[2].date, Some(tx2_date)); // tx3 gets tx2's date
        assert_eq!(sd.proto_transactions[3].date, Some(tx4_date)); // tx4 keeps its date
        assert_eq!(sd.proto_transactions[4].date, Some(tx4_date)); // tx5 gets tx4's date
    }

    #[test]
    fn test_fix_implicit_dates_preserves_existing_dates() {
        let mut sd = AccountData::new();
        let start_date = 1609459200000; // 2021-01-01 timestamp
        let existing_date = 1609545600000; // 2021-01-02 timestamp

        sd.set_start_date(start_date);

        // Transaction with date already set
        let mut tx1 = ProtoTransaction::new();
        tx1.set_date(existing_date);
        tx1.description = "Transaction with existing date".to_string();
        sd.add_proto_transaction(tx1);

        fix_implicit_dates(&mut sd);

        // Existing date should be preserved
        assert_eq!(sd.proto_transactions[0].date, Some(existing_date));
    }

    #[test]
    fn test_fix_implicit_dates_all_transactions_have_dates() {
        let mut sd = AccountData::new();
        let start_date = 1609459200000; // 2021-01-01 timestamp
        let tx1_date = 1609545600000; // 2021-01-02 timestamp
        let tx2_date = 1609632000000; // 2021-01-03 timestamp

        sd.set_start_date(start_date);

        // All transactions have dates
        let mut tx1 = ProtoTransaction::new();
        tx1.set_date(tx1_date);
        tx1.description = "Transaction 1".to_string();
        sd.add_proto_transaction(tx1);

        let mut tx2 = ProtoTransaction::new();
        tx2.set_date(tx2_date);
        tx2.description = "Transaction 2".to_string();
        sd.add_proto_transaction(tx2);

        fix_implicit_dates(&mut sd);

        // All dates should be preserved
        assert_eq!(sd.proto_transactions[0].date, Some(tx1_date));
        assert_eq!(sd.proto_transactions[1].date, Some(tx2_date));
    }

    #[test]
    fn test_fix_implicit_dates_first_transaction_has_date() {
        let mut sd = AccountData::new();
        let start_date = 1609459200000; // 2021-01-01 timestamp
        let tx1_date = 1609545600000; // 2021-01-02 timestamp (different from start)

        sd.set_start_date(start_date);

        // First transaction has date (different from start date)
        let mut tx1 = ProtoTransaction::new();
        tx1.set_date(tx1_date);
        tx1.description = "Transaction 1".to_string();
        sd.add_proto_transaction(tx1);

        // Second transaction has no date
        let mut tx2 = ProtoTransaction::new();
        tx2.description = "Transaction 2".to_string();
        sd.add_proto_transaction(tx2);

        fix_implicit_dates(&mut sd);

        // First transaction keeps its date, second gets first transaction's date
        assert_eq!(sd.proto_transactions[0].date, Some(tx1_date));
        assert_eq!(sd.proto_transactions[1].date, Some(tx1_date));
    }
}

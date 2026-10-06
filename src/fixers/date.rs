use crate::structs::AccountData;
use chrono::{DateTime, Datelike, Utc};

/// Fix transactions with year crossover dates.
///
/// This function handles cases where transaction dates appear to be from the previous year
/// due to year boundaries in statements. If a transaction date is before the statement start date,
/// it assumes the transaction actually occurred in the following year and adjusts accordingly.
pub fn fix_year_crossovers(sd: &mut AccountData) {
    // Return early if no start date
    let start_date = match sd.start_date {
        Some(date) => date,
        None => return,
    };

    // Get the year from the start date
    let start_datetime =
        DateTime::from_timestamp_millis(start_date).unwrap_or(DateTime::<Utc>::MIN_UTC);
    let year = start_datetime.year();

    for transaction in &mut sd.proto_transactions {
        if let Some(transaction_date) = transaction.date {
            // If transaction date is before start date, assume it's in the next year
            if transaction_date < start_date {
                // Convert to DateTime to manipulate the year
                if let Some(transaction_datetime) =
                    DateTime::from_timestamp_millis(transaction_date)
                {
                    // Create a new date with the next year (year + 1) to match TypeScript behavior
                    if let Some(new_datetime) = transaction_datetime.with_year(year + 1) {
                        transaction.set_date(new_datetime.timestamp_millis());
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::ProtoTransaction;
    use chrono::{DateTime, TimeZone, Utc};

    #[test]
    fn test_fix_year_crossovers_no_start_date() {
        let mut sd = AccountData::new();

        // Add a transaction with date
        let mut tx1 = ProtoTransaction::new();
        let tx_date = Utc
            .with_ymd_and_hms(2023, 12, 15, 0, 0, 0)
            .unwrap()
            .timestamp_millis();
        tx1.set_date(tx_date);
        sd.add_proto_transaction(tx1);

        // Should not panic when start date is None
        fix_year_crossovers(&mut sd);

        // Transaction date should remain unchanged
        assert_eq!(sd.proto_transactions[0].date, Some(tx_date));
    }

    #[test]
    fn test_fix_year_crossovers_no_transactions() {
        let mut sd = AccountData::new();
        let start_date = Utc
            .with_ymd_and_hms(2024, 1, 15, 0, 0, 0)
            .unwrap()
            .timestamp_millis();
        sd.set_start_date(start_date);

        // Should not panic with no transactions
        fix_year_crossovers(&mut sd);

        // Nothing should change
        assert_eq!(sd.proto_transactions.len(), 0);
    }

    #[test]
    fn test_fix_year_crossovers_transaction_after_start_date() {
        let mut sd = AccountData::new();
        let start_date = Utc
            .with_ymd_and_hms(2024, 1, 15, 0, 0, 0)
            .unwrap()
            .timestamp_millis();
        sd.set_start_date(start_date);

        // Transaction date after start date (should not change)
        let mut tx1 = ProtoTransaction::new();
        let tx_date = Utc
            .with_ymd_and_hms(2024, 2, 20, 0, 0, 0)
            .unwrap()
            .timestamp_millis();
        tx1.set_date(tx_date);
        sd.add_proto_transaction(tx1);

        fix_year_crossovers(&mut sd);

        // Transaction date should remain unchanged
        assert_eq!(sd.proto_transactions[0].date, Some(tx_date));
    }

    #[test]
    fn test_fix_year_crossovers_transaction_before_start_date() {
        let mut sd = AccountData::new();
        // Start date: January 15, 2024
        let start_date = Utc
            .with_ymd_and_hms(2024, 1, 15, 0, 0, 0)
            .unwrap()
            .timestamp_millis();
        sd.set_start_date(start_date);

        // Transaction date: December 25, 2023 (appears to be previous year)
        let mut tx1 = ProtoTransaction::new();
        let tx_date = Utc
            .with_ymd_and_hms(2023, 12, 25, 0, 0, 0)
            .unwrap()
            .timestamp_millis();
        tx1.set_date(tx_date);
        sd.add_proto_transaction(tx1);

        fix_year_crossovers(&mut sd);

        // Transaction should be moved to December 25, 2025 (year + 1 from start date year)
        let expected_date = Utc
            .with_ymd_and_hms(2025, 12, 25, 0, 0, 0)
            .unwrap()
            .timestamp_millis();
        assert_eq!(sd.proto_transactions[0].date, Some(expected_date));
    }

    #[test]
    fn test_fix_year_crossovers_multiple_transactions_mixed() {
        let mut sd = AccountData::new();
        // Start date: January 15, 2024
        let start_date = Utc
            .with_ymd_and_hms(2024, 1, 15, 0, 0, 0)
            .unwrap()
            .timestamp_millis();
        sd.set_start_date(start_date);

        // Transaction 1: Before start date (December 2023) - should be moved to 2025
        let mut tx1 = ProtoTransaction::new();
        let tx1_date = Utc
            .with_ymd_and_hms(2023, 12, 20, 0, 0, 0)
            .unwrap()
            .timestamp_millis();
        tx1.set_date(tx1_date);
        sd.add_proto_transaction(tx1);

        // Transaction 2: After start date (February 2024) - should remain unchanged
        let mut tx2 = ProtoTransaction::new();
        let tx2_date = Utc
            .with_ymd_and_hms(2024, 2, 10, 0, 0, 0)
            .unwrap()
            .timestamp_millis();
        tx2.set_date(tx2_date);
        sd.add_proto_transaction(tx2);

        // Transaction 3: Before start date (November 2023) - should be moved to 2025
        let mut tx3 = ProtoTransaction::new();
        let tx3_date = Utc
            .with_ymd_and_hms(2023, 11, 30, 0, 0, 0)
            .unwrap()
            .timestamp_millis();
        tx3.set_date(tx3_date);
        sd.add_proto_transaction(tx3);

        fix_year_crossovers(&mut sd);

        // Check results
        let expected_tx1_date = Utc
            .with_ymd_and_hms(2025, 12, 20, 0, 0, 0)
            .unwrap()
            .timestamp_millis();
        let expected_tx3_date = Utc
            .with_ymd_and_hms(2025, 11, 30, 0, 0, 0)
            .unwrap()
            .timestamp_millis();

        assert_eq!(sd.proto_transactions[0].date, Some(expected_tx1_date)); // tx1 moved to 2025
        assert_eq!(sd.proto_transactions[1].date, Some(tx2_date)); // tx2 unchanged
        assert_eq!(sd.proto_transactions[2].date, Some(expected_tx3_date)); // tx3 moved to 2025
    }

    #[test]
    fn test_fix_year_crossovers_transaction_without_date() {
        let mut sd = AccountData::new();
        let start_date = Utc
            .with_ymd_and_hms(2024, 1, 15, 0, 0, 0)
            .unwrap()
            .timestamp_millis();
        sd.set_start_date(start_date);

        // Transaction without date
        let mut tx1 = ProtoTransaction::new();
        tx1.description = "Transaction without date".to_string();
        sd.add_proto_transaction(tx1);

        // Transaction with date after start date
        let mut tx2 = ProtoTransaction::new();
        let tx2_date = Utc
            .with_ymd_and_hms(2024, 3, 10, 0, 0, 0)
            .unwrap()
            .timestamp_millis();
        tx2.set_date(tx2_date);
        sd.add_proto_transaction(tx2);

        fix_year_crossovers(&mut sd);

        // Transaction without date should remain None
        assert_eq!(sd.proto_transactions[0].date, None);
        // Transaction with date should remain unchanged (after start date)
        assert_eq!(sd.proto_transactions[1].date, Some(tx2_date));
    }

    #[test]
    fn test_fix_year_crossovers_same_day_as_start_date() {
        let mut sd = AccountData::new();
        // Start date: January 15, 2024
        let start_date = Utc
            .with_ymd_and_hms(2024, 1, 15, 0, 0, 0)
            .unwrap()
            .timestamp_millis();
        sd.set_start_date(start_date);

        // Transaction on exactly the same date
        let mut tx1 = ProtoTransaction::new();
        tx1.set_date(start_date);
        sd.add_proto_transaction(tx1);

        fix_year_crossovers(&mut sd);

        // Transaction should remain unchanged (not before start date)
        assert_eq!(sd.proto_transactions[0].date, Some(start_date));
    }

    #[test]
    fn test_fix_year_crossovers_one_millisecond_before() {
        let mut sd = AccountData::new();
        // Start date: January 15, 2024
        let start_date = Utc
            .with_ymd_and_hms(2024, 1, 15, 0, 0, 0)
            .unwrap()
            .timestamp_millis();
        sd.set_start_date(start_date);

        // Transaction one millisecond before start date
        let mut tx1 = ProtoTransaction::new();
        let tx_date = start_date - 1;
        tx1.set_date(tx_date);
        sd.add_proto_transaction(tx1);

        fix_year_crossovers(&mut sd);

        // Should be moved to next year
        let original_datetime = DateTime::from_timestamp_millis(tx_date).unwrap();
        let expected_date = original_datetime
            .with_year(2024 + 1)
            .unwrap()
            .timestamp_millis();
        assert_eq!(sd.proto_transactions[0].date, Some(expected_date));
    }

    #[test]
    fn test_fix_year_crossovers_year_boundary_december_january() {
        let mut sd = AccountData::new();
        // Start date: January 5, 2024 (early in year)
        let start_date = Utc
            .with_ymd_and_hms(2024, 1, 5, 0, 0, 0)
            .unwrap()
            .timestamp_millis();
        sd.set_start_date(start_date);

        // Transaction in December of previous year
        let mut tx1 = ProtoTransaction::new();
        let tx_date = Utc
            .with_ymd_and_hms(2023, 12, 31, 23, 59, 59)
            .unwrap()
            .timestamp_millis();
        tx1.set_date(tx_date);
        sd.add_proto_transaction(tx1);

        fix_year_crossovers(&mut sd);

        // Should be moved to December 31, 2025 (year + 1 from start date year)
        let expected_date = Utc
            .with_ymd_and_hms(2025, 12, 31, 23, 59, 59)
            .unwrap()
            .timestamp_millis();
        assert_eq!(sd.proto_transactions[0].date, Some(expected_date));
    }

    #[test]
    fn test_fix_year_crossovers_preserves_time_components() {
        let mut sd = AccountData::new();
        // Start date: January 15, 2024 at 12:30:45
        let start_date = Utc
            .with_ymd_and_hms(2024, 1, 15, 12, 30, 45)
            .unwrap()
            .timestamp_millis();
        sd.set_start_date(start_date);

        // Transaction: December 10, 2023 at 14:25:30
        let mut tx1 = ProtoTransaction::new();
        let tx_date = Utc
            .with_ymd_and_hms(2023, 12, 10, 14, 25, 30)
            .unwrap()
            .timestamp_millis();
        tx1.set_date(tx_date);
        sd.add_proto_transaction(tx1);

        fix_year_crossovers(&mut sd);

        // Should be moved to December 10, 2025 at 14:25:30 (time preserved)
        let expected_date = Utc
            .with_ymd_and_hms(2025, 12, 10, 14, 25, 30)
            .unwrap()
            .timestamp_millis();
        assert_eq!(sd.proto_transactions[0].date, Some(expected_date));
    }
}

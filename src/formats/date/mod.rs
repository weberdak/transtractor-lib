pub mod format1;
pub mod format10;
pub mod format11;
pub mod format12;
pub mod format13;
pub mod format14;
pub mod format15;
pub mod format16;
pub mod format2;
pub mod format3;
pub mod format4;
pub mod format5;
pub mod format6;
pub mod format7;
pub mod format8;
pub mod format9;
pub mod generate;

use crate::formats::date::generate::{parse_day, parse_month, parse_year};
use crate::formats::date::{
    format1::Format1, format2::Format2, format3::Format3, format4::Format4, format5::Format5,
    format6::Format6, format7::Format7, format8::Format8, format9::Format9, format10::Format10,
    format11::Format11, format12::Format12, format13::Format13, format14::Format14,
    format15::Format15, format16::Format16,
};

/// Trait for date formats.
pub trait DateFormat {
    /// Number of space-delimited items in the input string.
    fn num_items(&self) -> usize;

    /// Parse the input string and return a UTC timestamp (milliseconds since epoch) if valid.
    fn parse(&self, input: &str, year_str: &str, num_items: usize) -> Option<i64>;
}

/// Get a list of valid formats.
pub fn get_valid_formats() -> Vec<&'static str> {
    vec![
        "format1", "format2", "format3", "format4", "format5", "format6", "format7", "format8",
        "format9", "format10", "format11", "format12", "format13", "format14", "format15",
        "format16",
    ]
}

/// Stores day, month, and year strings and can convert to a UTC timestamp.
#[derive(Debug, Clone)]
pub struct DateParts {
    pub day_str: String,
    pub month_str: String,
    pub year_str: String,
}

impl DateParts {
    pub fn new(day_str: String, month_str: String, year_str: String) -> Self {
        Self {
            day_str,
            month_str,
            year_str,
        }
    }

    /// Attempts to convert the stored strings to a UTC timestamp (milliseconds since epoch).
    /// If self.year_str is empty, uses the input arg year_str.
    /// If both are empty, panics.
    /// If self.year_str is not empty, uses it even if the input arg is not empty.
    pub fn to_utc_timestamp(&self, year_str: &str) -> Option<i64> {
        let day = parse_day(&self.day_str)?;
        let month = parse_month(&self.month_str)?;

        // Determine which year string to use
        let year_source = if !self.year_str.trim().is_empty() {
            &self.year_str
        } else if !year_str.trim().is_empty() {
            year_str
        } else {
            panic!("No year string provided to to_utc_timestamp");
        };

        let year = parse_year(year_source)? as i32;

        // Try to create the date
        let date = chrono::NaiveDate::from_ymd_opt(year, month, day);

        // If parsing failed and we have February 29, try adding 1 year (leap year fix)
        let date = match date {
            Some(d) => d,
            None if day == 29 && month == 2 => {
                // Feb 29 failed, likely because current year is not a leap year
                // Try adding 1 year to handle year crossover issue with leap years
                chrono::NaiveDate::from_ymd_opt(year + 1, month, day)?
            }
            None => return None,
        };

        let datetime = date.and_hms_opt(0, 0, 0)?;
        Some(datetime.and_utc().timestamp_millis())
    }
}

/// Dispatcher for multiple date formats.
pub struct MultiDateFormatParser {
    parsers: Vec<Box<dyn DateFormat>>,
    item_counts: Vec<usize>,
}

impl MultiDateFormatParser {
    /// Create a new dispatcher from a list of format names.
    pub fn new(format_names: &[&str]) -> Self {
        // Collect (name, num_items) pairs
        let mut formats: Vec<(&str, usize)> = format_names
            .iter()
            .map(|&name| {
                let num_items = match name {
                    "format1" => Format1.num_items(),
                    "format2" => Format2.num_items(),
                    "format3" => Format3.num_items(),
                    "format4" => Format4.num_items(),
                    "format5" => Format5.num_items(),
                    "format6" => Format6.num_items(),
                    "format7" => Format7.num_items(),
                    "format8" => Format8.num_items(),
                    "format9" => Format9.num_items(),
                    "format10" => Format10.num_items(),
                    "format11" => Format11.num_items(),
                    "format12" => Format12.num_items(),
                    "format13" => Format13.num_items(),
                    "format14" => Format14.num_items(),
                    "format15" => Format15.num_items(),
                    "format16" => Format16.num_items(),
                    _ => 0,
                };
                (name, num_items)
            })
            .collect();

        // Sort by num_items descending
        formats.sort_by_key(|a| std::cmp::Reverse(a.1));

        // Instantiate parsers in sorted order
        let mut parsers: Vec<Box<dyn DateFormat>> = Vec::new();
        for &(name, _) in &formats {
            match name {
                "format1" => parsers.push(Box::new(Format1)),
                "format2" => parsers.push(Box::new(Format2)),
                "format3" => parsers.push(Box::new(Format3)),
                "format4" => parsers.push(Box::new(Format4)),
                "format5" => parsers.push(Box::new(Format5)),
                "format6" => parsers.push(Box::new(Format6)),
                "format7" => parsers.push(Box::new(Format7)),
                "format8" => parsers.push(Box::new(Format8)),
                "format9" => parsers.push(Box::new(Format9)),
                "format10" => parsers.push(Box::new(Format10)),
                "format11" => parsers.push(Box::new(Format11)),
                "format12" => parsers.push(Box::new(Format12)),
                "format13" => parsers.push(Box::new(Format13)),
                "format14" => parsers.push(Box::new(Format14)),
                "format15" => parsers.push(Box::new(Format15)),
                "format16" => parsers.push(Box::new(Format16)),
                _ => {}
            }
        }
        let mut item_counts = parsers
            .iter()
            .map(|parser| parser.num_items())
            .collect::<Vec<_>>();
        item_counts.sort_unstable_by(|left, right| right.cmp(left));
        item_counts.dedup();
        MultiDateFormatParser {
            parsers,
            item_counts,
        }
    }

    /// Try parsing with each format in order, returning the first successful result.
    pub fn parse(&self, input: &str, year_str: &str, num_items: usize) -> Option<i64> {
        for parser in &self.parsers {
            if let Some(val) = parser.parse(input, year_str, num_items) {
                return Some(val);
            }
        }
        None
    }

    /// Returns the maximum number of items among all formats.
    pub fn max_items(&self) -> usize {
        self.item_counts.first().copied().unwrap_or(0)
    }

    /// Return distinct supported item counts, largest first.
    pub fn item_counts(&self) -> &[usize] {
        &self.item_counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_date() {
        let dp = DateParts {
            day_str: "15".to_string(),
            month_str: "Feb".to_string(),
            year_str: "2023".to_string(),
        };
        assert_eq!(dp.to_utc_timestamp(""), Some(1676419200000)); // 2023-02-15T00:00:00Z
    }

    #[test]
    fn test_invalid_date() {
        let dp = DateParts {
            day_str: "32".to_string(),
            month_str: "Feb".to_string(),
            year_str: "2023".to_string(),
        };
        assert_eq!(dp.to_utc_timestamp(""), None);
    }

    #[test]
    fn test_invalid_month() {
        let dp = DateParts {
            day_str: "15".to_string(),
            month_str: "Foo".to_string(),
            year_str: "2023".to_string(),
        };
        assert_eq!(dp.to_utc_timestamp(""), None);
    }

    #[test]
    fn test_invalid_year() {
        let dp = DateParts {
            day_str: "15".to_string(),
            month_str: "Feb".to_string(),
            year_str: "abcd".to_string(),
        };
        assert_eq!(dp.to_utc_timestamp(""), None);
    }

    #[test]
    fn test_to_utc_timestamp_uses_input_year_if_self_year_empty() {
        let dp = DateParts {
            day_str: "15".to_string(),
            month_str: "Feb".to_string(),
            year_str: "".to_string(),
        };
        assert_eq!(dp.to_utc_timestamp("2023"), Some(1676419200000)); // 2023-02-15T00:00:00Z
    }

    #[test]
    #[should_panic(expected = "No year string provided to to_utc_timestamp")]
    fn test_to_utc_timestamp_panics_if_no_year_provided() {
        let dp = DateParts {
            day_str: "15".to_string(),
            month_str: "Feb".to_string(),
            year_str: "".to_string(),
        };
        dp.to_utc_timestamp("");
    }

    #[test]
    fn test_multi_date_format_parser() {
        let multi_fmt = MultiDateFormatParser::new(&[
            "format1", "format2", "format3", "format4", "format5", "format6", "format7", "format8",
            "format9", "format10", "format11", "format12", "format13", "format16",
        ]);
        // Should parse using format1
        assert!(multi_fmt.parse("24 mar", "2023", 2).is_some());
        // Should parse using format2
        assert!(multi_fmt.parse("24 march 2020", "", 3).is_some());
        // Should parse using format3
        assert!(multi_fmt.parse("march 24, 2020", "", 3).is_some());
        // Should parse using format4
        assert!(multi_fmt.parse("24/3/2020", "", 1).is_some());
        // Should parse using format5
        assert!(multi_fmt.parse("24/3/20", "", 1).is_some());
        // Should parse using format6
        assert!(multi_fmt.parse("3/24", "2020", 1).is_some());
        // Should parse using format7
        assert!(multi_fmt.parse("24-03-2020", "", 1).is_some());
        assert!(multi_fmt.parse("24-3-20", "", 1).is_some());
        // Should parse using format8
        assert!(multi_fmt.parse("03-24-2020", "", 1).is_some());
        assert!(multi_fmt.parse("3-24-20", "", 1).is_some());
        // Should parse using format9
        assert!(multi_fmt.parse("03/24/2020", "", 1).is_some());
        assert!(multi_fmt.parse("3/24/20", "", 1).is_some());
        // Should parse using format10
        assert!(multi_fmt.parse("Mar 24", "2023", 2).is_some());
        assert!(multi_fmt.parse("March 24", "2023", 2).is_some());
        assert!(multi_fmt.parse("March 4", "2023", 2).is_some());
        // Should parse using format11
        assert!(
            multi_fmt
                .parse("Mar 24, 2023-Apr 24, 2023", "", 5)
                .is_some()
        );
        assert!(
            multi_fmt
                .parse("March 1, 2020-March 31, 2020", "", 5)
                .is_some()
        );
        // Should parse using format12
        assert!(multi_fmt.parse("2023/03/24", "", 1).is_some());
        assert!(multi_fmt.parse("2023/3/24", "", 1).is_some());
        // Should parse using format13
        assert!(multi_fmt.parse("2023-03-24", "", 1).is_some());
        assert!(multi_fmt.parse("2023-3-24", "", 1).is_some());
        // Should parse using format16
        assert!(multi_fmt.parse("24 march 20", "", 3).is_some());
        // Should not parse invalid
        assert_eq!(multi_fmt.parse("foo", "2023", 1), None);
    }

    #[test]
    fn test_february_29_leap_year_fix() {
        // Test that Feb 29 in a non-leap year gets corrected to the next leap year
        let dp = DateParts {
            day_str: "29".to_string(),
            month_str: "Feb".to_string(),
            year_str: "2023".to_string(), // 2023 is not a leap year
        };

        // Should automatically try 2024 (which is a leap year) when 2023 fails
        let result = dp.to_utc_timestamp("");
        assert!(result.is_some());

        // Verify it's actually 2024-02-29
        let expected_2024_feb_29 = chrono::NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_millis();
        assert_eq!(result.unwrap(), expected_2024_feb_29);
    }

    #[test]
    fn test_february_29_already_leap_year() {
        // Test that Feb 29 in an actual leap year works normally
        let dp = DateParts {
            day_str: "29".to_string(),
            month_str: "Feb".to_string(),
            year_str: "2024".to_string(), // 2024 is a leap year
        };

        let result = dp.to_utc_timestamp("");
        assert!(result.is_some());

        // Should be exactly 2024-02-29
        let expected_2024_feb_29 = chrono::NaiveDate::from_ymd_opt(2024, 2, 29)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_millis();
        assert_eq!(result.unwrap(), expected_2024_feb_29);
    }

    #[test]
    fn test_max_items() {
        let multi_fmt = MultiDateFormatParser::new(&["format1", "format3", "format5"]);
        assert_eq!(multi_fmt.max_items(), 3);

        let multi_fmt2 = MultiDateFormatParser::new(&["format1"]);
        assert_eq!(multi_fmt2.max_items(), 2);

        let multi_fmt3 = MultiDateFormatParser::new(&[]);
        assert_eq!(multi_fmt3.max_items(), 0);
    }
}

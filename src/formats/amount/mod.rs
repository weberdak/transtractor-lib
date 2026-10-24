pub mod format1;
pub mod format10;
pub mod format2;
pub mod format3;
pub mod format4;
pub mod format5;
pub mod format6;
pub mod format7;
pub mod format8;
pub mod format9;

use format1::Format1;
use format2::Format2;
use format3::Format3;
use format4::Format4;
use format5::Format5;
use format6::Format6;
use format7::Format7;
use format8::Format8;
use format9::Format9;
use format10::Format10;

/// Trait for amount formats.
pub trait AmountFormat {
    /// Number of space-delimited items in the input string.
    fn num_items(&self) -> usize;

    /// Parse the input string and return a float if valid.
    fn parse(&self, input: &str, num_items: usize) -> Option<f64>;
}

/// Get a list of valid formats.
pub fn get_valid_formats() -> Vec<&'static str> {
    vec![
        "format1", "format2", "format3", "format4", "format5", "format6", "format7", "format8",
        "format9", "format10",
    ]
}

/// Dispatcher for multiple amount formats.
pub struct MultiAmountFormatParser {
    parsers: Vec<Box<dyn AmountFormat>>,
    item_counts: Vec<usize>,
}

impl MultiAmountFormatParser {
    /// Create a new dispatcher from a list of format names.
    pub fn new(format_names: &[&str]) -> Self {
        // Collect (name, NUM_TERMS) pairs
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
                    _ => 0,
                };
                (name, num_items)
            })
            .collect();

        // Sort by num items descending
        formats.sort_by_key(|a| std::cmp::Reverse(a.1));

        // Instantiate parsers in sorted order
        let mut parsers: Vec<Box<dyn AmountFormat>> = Vec::new();
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
                _ => {}
            }
        }
        let mut item_counts = parsers
            .iter()
            .map(|parser| parser.num_items())
            .collect::<Vec<_>>();
        item_counts.sort_unstable_by(|left, right| right.cmp(left));
        item_counts.dedup();
        MultiAmountFormatParser {
            parsers,
            item_counts,
        }
    }

    /// Try parsing with each format in order, returning the first successful result.
    pub fn parse(&self, input: &str, num_items: usize) -> Option<f64> {
        for parser in &self.parsers {
            if let Some(val) = parser.parse(input, num_items) {
                return Some(val);
            }
        }
        None
    }

    /// Get the maximum number of items among the included formats.
    pub fn max_items(&self) -> usize {
        self.item_counts.first().copied().unwrap_or(0)
    }

    /// Return distinct supported item counts, largest first.
    pub fn item_counts(&self) -> &[usize] {
        &self.item_counts
    }
}

// Example usage:
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multi_amount_format_parser() {
        let multi_fmt1 = MultiAmountFormatParser::new(&["format1", "format2"]);
        assert_eq!(multi_fmt1.parse("1,234.56", 1), Some(1234.56));
        assert_eq!(multi_fmt1.parse("-$1,234.56", 1), Some(-1234.56)); // format2
        assert_eq!(multi_fmt1.parse("$1,234.56 DR", 2), None); // format3 not included
    }

    #[test]
    fn test_max_items() {
        let multi_fmt = MultiAmountFormatParser::new(&["format1", "format3", "format5"]);
        // format1: 1 item, format3: 2 items, format5: 1 item
        assert_eq!(multi_fmt.max_items(), 2);

        let multi_fmt2 = MultiAmountFormatParser::new(&["format1", "format5"]);
        assert_eq!(multi_fmt2.max_items(), 1);

        let multi_fmt3 = MultiAmountFormatParser::new(&[]);
        assert_eq!(multi_fmt3.max_items(), 0);
    }
}

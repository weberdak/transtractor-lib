use crate::formats::amount::AmountFormat;
use regex::Regex;

/// Format8: parses amounts like "-$1,234 56", "$1,234 56", "$1,234 56-"
/// (same as Format2, but the decimal point is replaced by a single space)
pub struct Format8;

impl AmountFormat for Format8 {
    fn num_items(&self) -> usize {
        2
    }

    fn parse(&self, currency_str: &str, num_items: usize) -> Option<f64> {
        if num_items != self.num_items() {
            return None;
        }
        let re = Regex::new(r"^-?\$\d{1,3}(,\d{3})* \d{2}-?$").unwrap();
        if !re.is_match(currency_str) {
            return None;
        }
        let sign = if currency_str.contains('-') {
            -1.0
        } else {
            1.0
        };
        let cleaned = currency_str.replace([',', '-', '$'], "").replace(' ', ".");
        cleaned.parse::<f64>().ok().map(|val| sign * val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format8() {
        let fmt = Format8;
        assert_eq!(fmt.parse("$1,234 56", 2), Some(1234.56));
        assert_eq!(fmt.parse("$1,234 56-", 2), Some(-1234.56));
        assert_eq!(fmt.parse("-$1,234 56", 2), Some(-1234.56));
        assert_eq!(fmt.parse("$1,234 56-", 2), Some(-1234.56));
        assert_eq!(fmt.parse("$1,000,234 56", 2), Some(1000234.56));
        assert_eq!(fmt.parse("$1,234 56", 1), None);
        assert_eq!(fmt.parse("$1,234.56", 2), None);
        assert_eq!(fmt.parse("$1234 56", 2), None);
        assert_eq!(fmt.parse("$1,234 5", 2), None);
        assert_eq!(fmt.parse("$1,234  56", 2), None);
        assert_eq!(fmt.parse("bad input", 2), None);
    }
}

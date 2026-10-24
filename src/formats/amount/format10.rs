use crate::formats::amount::AmountFormat;
use regex::Regex;

/// Format10: parses amounts like "-$1,23456", "$1,23456", "$1,23456-"
/// (same as Format8, but the decimal point is missing, so the cents are merged
/// into the main number and are recovered as the last two digits)
pub struct Format10;

impl AmountFormat for Format10 {
    fn num_items(&self) -> usize {
        1
    }

    fn parse(&self, currency_str: &str, num_items: usize) -> Option<f64> {
        if num_items != self.num_items() {
            return None;
        }
        let re = Regex::new(r"^-?\$\d{1,3}(,\d{3})*\d{2}-?$").unwrap();
        if !re.is_match(currency_str) {
            return None;
        }
        let sign = if currency_str.contains('-') {
            -1.0
        } else {
            1.0
        };
        let mut digits = currency_str.replace([',', '-', '$'], "");
        digits.insert(digits.len() - 2, '.');
        digits.parse::<f64>().ok().map(|val| sign * val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format10() {
        let fmt = Format10;
        assert_eq!(fmt.parse("$1,23456", 1), Some(1234.56));
        assert_eq!(fmt.parse("-$1,23456", 1), Some(-1234.56));
        assert_eq!(fmt.parse("$1,23456-", 1), Some(-1234.56));
        assert_eq!(fmt.parse("$1,000,23456", 1), Some(1000234.56));
        assert_eq!(fmt.parse("$12345", 1), Some(123.45));
        assert_eq!(fmt.parse("$123", 1), Some(1.23));
        assert_eq!(fmt.parse("$1,23456", 2), None);
        assert_eq!(fmt.parse("$1,234.56", 1), None);
        assert_eq!(fmt.parse("$1,234 56", 1), None);
        assert_eq!(fmt.parse("1,23456", 1), None);
        assert_eq!(fmt.parse("$1234567", 1), None);
        assert_eq!(fmt.parse("$12", 1), None);
        assert_eq!(fmt.parse("bad input", 1), None);
    }
}

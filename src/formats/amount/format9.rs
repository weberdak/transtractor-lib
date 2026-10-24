use crate::formats::amount::AmountFormat;
use regex::Regex;

/// Format9: parses amounts like "1,234 56", "-1,234 56", "1,234 56-"
/// (same as Format1, but the decimal point is replaced by a single space)
pub struct Format9;

impl AmountFormat for Format9 {
    fn num_items(&self) -> usize {
        2
    }

    fn parse(&self, amount_str: &str, num_items: usize) -> Option<f64> {
        if num_items != self.num_items() {
            return None;
        }
        let re = Regex::new(r"^-?\d{1,3}(,\d{3})* \d{2}-?$").unwrap();
        if !re.is_match(amount_str) {
            return None;
        }
        let sign = if amount_str.contains('-') { -1.0 } else { 1.0 };
        let cleaned = amount_str.replace([',', '-'], "").replace(' ', ".");
        cleaned.parse::<f64>().ok().map(|val| sign * val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format9() {
        let fmt = Format9;
        assert_eq!(fmt.parse("1,234 56", 2), Some(1234.56));
        assert_eq!(fmt.parse("-1,234 56", 2), Some(-1234.56));
        assert_eq!(fmt.parse("1,234 56-", 2), Some(-1234.56));
        assert_eq!(fmt.parse("1,000,234 56", 2), Some(1000234.56));
        assert_eq!(fmt.parse("1,234 56", 1), None);
        assert_eq!(fmt.parse("1,234.56", 2), None);
        assert_eq!(fmt.parse("$1,234 56", 2), None);
        assert_eq!(fmt.parse("1234 56", 2), None);
        assert_eq!(fmt.parse("1,234 5", 2), None);
        assert_eq!(fmt.parse("1,234  56", 2), None);
        assert_eq!(fmt.parse("bad input", 2), None);
    }
}

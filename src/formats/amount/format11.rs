use crate::formats::amount::AmountFormat;
use regex::Regex;

/// Format11: parses amounts like "-$1,234.56DR", "$1,234.56DR", "$1,234.56CR"
/// (same as Format3, but without a space before CR/DR)
pub struct Format11;

impl AmountFormat for Format11 {
    fn num_items(&self) -> usize {
        1
    }

    fn parse(&self, currency_str: &str, num_items: usize) -> Option<f64> {
        if num_items != self.num_items() {
            return None;
        }
        let currency_str = currency_str.to_lowercase();
        let re = Regex::new(r"^-?\$\d{1,3}(,\d{3})*\.\d{2}(cr|dr)$").unwrap();
        if !re.is_match(&currency_str) {
            return None;
        }
        let mut sign = if currency_str.ends_with("dr") {
            -1.0
        } else {
            1.0
        };
        if currency_str.starts_with('-') {
            sign *= -1.0;
        }
        let cleaned = currency_str.replace(['$', ',', '-'], "");
        let cleaned = &cleaned[..cleaned.len() - 2];
        cleaned.parse::<f64>().ok().map(|val| sign * val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format11() {
        let fmt = Format11;
        assert_eq!(fmt.parse("$1,234.56DR", 1), Some(-1234.56));
        assert_eq!(fmt.parse("-$1,234.56DR", 1), Some(1234.56));
        assert_eq!(fmt.parse("$1,234.56CR", 1), Some(1234.56));
        assert_eq!(fmt.parse("$1,234.56cr", 1), Some(1234.56));
        assert_eq!(fmt.parse("$1,000,234.56CR", 1), Some(1000234.56));
        assert_eq!(fmt.parse("$4.00DR", 1), Some(-4.00));
        assert_eq!(fmt.parse("$1,234.56 DR", 1), None);
        assert_eq!(fmt.parse("$1,234.56DR", 2), None);
        assert_eq!(fmt.parse("1234.56DR", 1), None);
        assert_eq!(fmt.parse("$1234.56DR", 1), None);
        assert_eq!(fmt.parse("$1,234.5DR", 1), None);
        assert_eq!(fmt.parse("$1,234.567DR", 1), None);
        assert_eq!(fmt.parse("bad input", 1), None);
    }
}

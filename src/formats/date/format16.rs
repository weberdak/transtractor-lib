use crate::formats::date::DateFormat;
use crate::formats::date::DateParts;

/// Format16: parses dates like "24 march 20", "1 march 20", "1 mar 20"
pub struct Format16;

impl DateFormat for Format16 {
    fn num_items(&self) -> usize {
        3
    }

    /// Parses a date string and returns the UTC timestamp if valid.
    fn parse(&self, date_str: &str, _year_str: &str, num_items: usize) -> Option<i64> {
        if num_items != self.num_items() {
            return None;
        }
        let re = regex::Regex::new(r"^\d{1,2} \w+ \d{2}$").unwrap();
        if !re.is_match(date_str) {
            return None;
        }
        let parts: Vec<&str> = date_str.split(' ').collect();
        if parts.len() != 3 {
            return None;
        }
        let date_parts = DateParts {
            day_str: parts[0].to_string(),
            month_str: parts[1].to_string(),
            year_str: parts[2].to_string(),
        };
        date_parts.to_utc_timestamp("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format16_parse() {
        let fmt = Format16;
        assert!(fmt.parse("24 march 20", "", 3).is_some());
        assert!(fmt.parse("1 mar 20", "", 3).is_some());
        assert_eq!(
            fmt.parse("24 march 20", "", 3),
            crate::formats::date::format2::Format2.parse("24 march 2020", "", 3)
        );
        assert_eq!(fmt.parse("24 march 2020", "", 3), None);
        assert_eq!(fmt.parse("24 march 2", "", 3), None);
        assert_eq!(fmt.parse("24 march 20", "", 2), None);
        assert_eq!(fmt.parse("march 24 20", "", 3), None);
        assert_eq!(fmt.parse("24 invalidmonth 20", "", 3), None);
    }
}

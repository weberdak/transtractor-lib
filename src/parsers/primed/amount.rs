use crate::parsers::base::AmountParser;
use crate::parsers::base::ParserPrimer;
use crate::structs::TextItem;
use crate::structs::benchmark::Timer;

pub struct PrimedAmountParser {
    primer_parser: ParserPrimer,
    amount_parser: AmountParser,
    alignment: String,
    alignment_tol: i32,
    invert: bool,
}

impl PrimedAmountParser {
    pub fn new(
        primer_terms: &[&str],
        amount_formats: &[&str],
        alignment: &str,
        alignment_tol: i32,
        invert: bool,
        trigger_count: usize,
    ) -> Self {
        Self {
            primer_parser: ParserPrimer::new(primer_terms, trigger_count),
            amount_parser: AmountParser::new(amount_formats),
            alignment: alignment.to_string(),
            alignment_tol,
            invert,
        }
    }

    pub fn parse_items(&mut self, items: &[TextItem]) -> usize {
        self.parse_items_inner(items, None)
    }

    pub fn parse_items_timed(&mut self, items: &[TextItem], prime_timer: &mut Timer) -> usize {
        self.parse_items_inner(items, Some(prime_timer))
    }

    fn parse_items_inner(&mut self, items: &[TextItem], prime_timer: Option<&mut Timer>) -> usize {
        // No items to parse
        if items.is_empty() {
            return 0;
        }

        // Return if value already set
        if self.amount_parser.value.is_some() {
            return 0;
        }

        // Primer terms are visible to other parsers, including after this
        // parser is primed.
        if self.primer_parser.primed {
            if self.primer_parser.matching_items(items) > 0 {
                return 0;
            }
        } else {
            let matched = match prime_timer {
                Some(timer) => self.primer_parser.parse_items_timed(items, timer),
                None => self.primer_parser.parse_items(items),
            };
            if matched > 0 {
                return 0;
            }
        }

        // Must be primed to look for amount
        if !self.primer_parser.primed {
            return 0; // Primer not found yet
        }

        // Primer is primed, look for amount
        let consumed = self.amount_parser.parse_items(items);
        if consumed == 0 {
            return 0; // No amount found
        }

        // Both primer and date found, check conditions
        let amount_item = self.amount_parser.text_item.as_ref().unwrap();

        // When primed via set_first_match there is no primer text item to align
        // against, so skip alignment/page checks in that case.
        let (valid_alignment, page_ok) = match self.primer_parser.text_item.as_ref() {
            Some(primer_item) => {
                let valid_alignment = match self.alignment.as_str() {
                    "x1" => (amount_item.x1 - primer_item.x1).abs() <= self.alignment_tol,
                    "x2" => (amount_item.x2 - primer_item.x2).abs() <= self.alignment_tol,
                    "y1" => (amount_item.y1 - primer_item.y1).abs() <= self.alignment_tol,
                    "y2" => (amount_item.y2 - primer_item.y2).abs() <= self.alignment_tol,
                    "" => true, // No alignment check
                    _ => true,  // No alignment check
                };
                (valid_alignment, amount_item.page == primer_item.page)
            }
            None => (true, true),
        };

        // Return 0 if any condition fails
        if !valid_alignment || !page_ok {
            // Reset amount parser state
            self.amount_parser.reset();
            return 0;
        }

        // All conditions met, finalize
        if self.invert {
            self.amount_parser.invert();
        }
        consumed
    }

    pub fn value(&self) -> Option<f64> {
        self.amount_parser.value
    }

    /// Whether the primer term has been matched
    pub fn is_primed(&self) -> bool {
        self.primer_parser.primed
    }

    /// Get the highest lookahead between primer and amount parsers
    pub fn get_max_lookahead(&self) -> usize {
        self.primer_parser
            .max_lookahead
            .max(self.amount_parser.max_lookahead)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::TextItem;

    fn make_text_item(text: &str, x1: i32, y1: i32, page: i32) -> TextItem {
        TextItem {
            text: text.to_string(),
            x1,
            y1,
            x2: x1 + 10,
            y2: y1 + 10,
            page,
            y1_bin: y1,
        }
    }

    #[test]
    fn test_primer_and_amount_success() {
        let mut parser = PrimedAmountParser::new(&["PRIME"], &["format1"], "x1", 5, false, 1);
        let items = vec![
            make_text_item("PRIME", 100, 200, 1),
            make_text_item("1,234.56", 102, 202, 1),
        ];
        // First call primes the parser
        let consumed_primer = parser.parse_items(&items);
        assert_eq!(consumed_primer, 0);
        assert!(parser.primer_parser.primed);
        assert!(parser.amount_parser.value.is_none());
        assert_eq!(parser.parse_items(&items), 0);

        // Second call parses the amount
        let consumed_amount = parser.parse_items(&items[1..]);
        assert_eq!(consumed_amount, 1);
        assert_eq!(parser.amount_parser.value, Some(1234.56));
    }

    #[test]
    fn test_primer_and_amount_invert() {
        let mut parser = PrimedAmountParser::new(&["PRIME"], &["format1"], "x1", 5, true, 1);
        let items = vec![
            make_text_item("PRIME", 100, 200, 1),
            make_text_item("1,234.56", 100, 200, 1),
        ];
        parser.parse_items(&items);
        let consumed = parser.parse_items(&items[1..]);
        assert_eq!(consumed, 1);
        assert_eq!(parser.amount_parser.value, Some(-1234.56));
    }

    #[test]
    fn test_primer_x1_fail() {
        let mut parser = PrimedAmountParser::new(&["PRIME"], &["format1"], "x1", 1, false, 1);
        let items = vec![
            make_text_item("PRIME", 100, 200, 1),
            make_text_item("1,234.56", 105, 200, 1),
        ];
        parser.parse_items(&items);
        let consumed = parser.parse_items(&items[1..]);
        assert_eq!(consumed, 0);
        assert!(parser.amount_parser.value.is_none());
    }

    #[test]
    fn test_primer_y1_fail() {
        let mut parser = PrimedAmountParser::new(&["PRIME"], &["format1"], "y1", 1, false, 1);
        let items = vec![
            make_text_item("PRIME", 100, 200, 1),
            make_text_item("1,234.56", 100, 205, 1),
        ];
        parser.parse_items(&items);
        let consumed = parser.parse_items(&items[1..]);
        assert_eq!(consumed, 0);
        assert!(parser.amount_parser.value.is_none());
    }

    #[test]
    fn test_primer_page_fail() {
        let mut parser = PrimedAmountParser::new(&["PRIME"], &["format1"], "", 0, false, 1);
        let items = vec![
            make_text_item("PRIME", 100, 200, 1),
            make_text_item("1,234.56", 100, 200, 2),
        ];
        parser.parse_items(&items);
        let consumed = parser.parse_items(&items[1..]);
        assert_eq!(consumed, 0);
        assert!(parser.amount_parser.value.is_none());
    }

    #[test]
    fn test_no_items() {
        let mut parser = PrimedAmountParser::new(&["PRIME"], &["format1"], "x1", 5, false, 1);
        let items: Vec<TextItem> = vec![];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 0);
        assert!(!parser.primer_parser.primed);
        assert!(parser.amount_parser.value.is_none());
    }

    #[test]
    fn test_amount_already_set() {
        let mut parser = PrimedAmountParser::new(&["PRIME"], &["format1"], "x1", 5, false, 1);
        let items = vec![
            make_text_item("PRIME", 100, 200, 1),
            make_text_item("1,234.56", 100, 200, 1),
        ];
        parser.parse_items(&items);
        parser.parse_items(&items[1..]);
        // Try parsing again, should return 0 since value is already set
        let consumed = parser.parse_items(&items[1..]);
        assert_eq!(consumed, 0);
    }

    #[test]
    fn test_primer_match_is_not_consumed_after_priming() {
        let mut parser = PrimedAmountParser::new(&["PRIME"], &["format1"], "", 0, false, 1);
        let primer = vec![make_text_item("PRIME", 100, 200, 1)];
        assert_eq!(parser.parse_items(&primer), 0);
        assert!(parser.is_primed());
        assert_eq!(parser.parse_items(&primer), 0);
        assert!(parser.value().is_none());
    }
}

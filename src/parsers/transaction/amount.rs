use crate::parsers::base::{AmountParser, ParserPrimer};
use crate::structs::{ProtoTransaction, StatementConfig, TextItem};

pub struct TransactionAmountParser {
    pub primed: bool,
    amount_parser: AmountParser,
    header_primer: ParserPrimer,
    invert_header_primer: ParserPrimer,
    alignment: String,
    invert_alignment: String,
    x1_range: Vec<i32>,
    x2_range: Vec<i32>,
    invert_x1_range: Vec<i32>,
    invert_x2_range: Vec<i32>,
    has_inverted_column: bool,
    x_tol: i32,
    invert: bool,
}

impl TransactionAmountParser {
    pub fn new(config: &StatementConfig) -> Self {
        let primer_terms: Vec<&str> = config
            .transaction_amount_headers
            .iter()
            .map(|s| s.as_str())
            .collect();
        let invert_primer_terms: Vec<&str> = config
            .transaction_amount_invert_headers
            .iter()
            .map(|s| s.as_str())
            .collect();
        let amount_formats: Vec<&str> = config
            .transaction_amount_formats
            .iter()
            .map(|s| s.as_str())
            .collect();
        let alignment = config.transaction_amount_alignment.clone();
        let invert_alignment = config.transaction_amount_invert_alignment.clone();
        let x_tol = config.transaction_alignment_tol;
        Self {
            primed: false,
            amount_parser: AmountParser::new(amount_formats.as_slice()),
            header_primer: ParserPrimer::new(primer_terms.as_slice(), 1),
            invert_header_primer: ParserPrimer::new(invert_primer_terms.as_slice(), 1),
            alignment,
            invert_alignment,
            x_tol,
            x1_range: config.transaction_amount_x1_range.to_vec(),
            x2_range: config.transaction_amount_x2_range.to_vec(),
            invert_x1_range: config.transaction_amount_invert_x1_range.to_vec(),
            invert_x2_range: config.transaction_amount_invert_x2_range.to_vec(),
            has_inverted_column: !invert_primer_terms.is_empty(),
            invert: config.transaction_amount_invert,
        }
    }

    pub fn parse_items(&mut self, items: &[TextItem], transaction: &mut ProtoTransaction) -> usize {
        // Try reading and setting bounds from header
        let header_consumed = self.try_parse_header(items);
        if header_consumed > 0 {
            return header_consumed;
        }

        // Try reading and setting bounds from invert header
        let invert_header_consumed = self.try_parse_invert_header(items);
        if invert_header_consumed > 0 {
            return invert_header_consumed;
        }

        // Parser must be primed before parsing amounts
        if !self.primed {
            return 0;
        }

        // Try parsing amount
        let amount_consumed = self.try_parse_amount(items);
        if amount_consumed > 0 {
            let mut value = self.amount_parser.value.unwrap();
            if self.invert {
                value = -value;
            }
            transaction.amount = Some(value);
            return amount_consumed;
        }
        0
    }

    /// Reset the parser state
    pub fn reset(&mut self) {
        self.primed = false;
        self.amount_parser.reset();
    }

    /// Set parser as primed
    pub fn prime(&mut self) {
        self.primed = true;
    }

    /// Get the maximum lookahead for the parser
    pub fn get_max_lookahead(&self) -> usize {
        let mut max_lookahead = 0;
        max_lookahead = max_lookahead.max(self.header_primer.max_lookahead);
        max_lookahead = max_lookahead.max(self.amount_parser.max_lookahead);
        max_lookahead
    }

    /// Check if header is set
    pub fn is_header_set(&self) -> bool {
        if self.has_inverted_column {
            return self.header_primer.primed && self.invert_header_primer.primed;
        }
        self.header_primer.primed
    }

    /// Get effective x_bounds
    pub fn get_x_bounds(&self) -> (i32, i32) {
        let mut x_lower = 0;
        let mut x_upper = 10000;
        if self.alignment == "x1" {
            x_lower = self.x1_range[0];
            x_upper = self.x1_range[1];
        } else if self.alignment == "x2" {
            x_lower = self.x2_range[0];
            x_upper = self.x2_range[1];
        }
        if self.has_inverted_column {
            let (inv_lower, inv_upper) = if self.invert_alignment == "x1" {
                (self.invert_x1_range[0], self.invert_x1_range[1])
            } else {
                (self.invert_x2_range[0], self.invert_x2_range[1])
            };
            x_lower = x_lower.min(inv_lower);
            x_upper = x_upper.max(inv_upper);
        }
        (x_lower, x_upper)
    }

    /// Try reading header and set x_ranges accordingly
    fn try_parse_header(&mut self, items: &[TextItem]) -> usize {
        // Return if header already read
        if self.header_primer.primed {
            return 0;
        }
        let header_consumed = self.header_primer.parse_items(items);
        if header_consumed > 0 {
            let item = self.header_primer.text_item.as_ref().unwrap();
            if self.alignment == "x1" {
                self.x1_range = vec![item.x1 - self.x_tol, item.x1 + self.x_tol];
            } else if self.alignment == "x2" {
                self.x2_range = vec![item.x2 - self.x_tol, item.x2 + self.x_tol];
            }
        }
        header_consumed
    }

    /// Try reading invert header and set invert_x_ranges accordingly
    fn try_parse_invert_header(&mut self, items: &[TextItem]) -> usize {
        // Return if invert header already read or no invert column configured
        if self.invert_header_primer.primed || !self.has_inverted_column {
            return 0;
        }
        let header_consumed = self.invert_header_primer.parse_items(items);
        if header_consumed > 0 {
            let item = self.invert_header_primer.text_item.as_ref().unwrap();
            if self.invert_alignment == "x1" {
                self.invert_x1_range = vec![item.x1 - self.x_tol, item.x1 + self.x_tol];
            } else if self.invert_alignment == "x2" {
                self.invert_x2_range = vec![item.x2 - self.x_tol, item.x2 + self.x_tol];
            }
        }
        header_consumed
    }

    /// Try parsing amount, invert if within invert bounds
    fn try_parse_amount(&mut self, items: &[TextItem]) -> usize {
        let x1_range = (self.x1_range[0], self.x1_range[1]);
        let x2_range = (self.x2_range[0], self.x2_range[1]);
        let invert_x1_range = (self.invert_x1_range[0], self.invert_x1_range[1]);
        let invert_x2_range = (self.invert_x2_range[0], self.invert_x2_range[1]);
        let has_inverted_column = self.has_inverted_column;
        // Reject candidates outside both the normal and (if configured) invert ranges
        // before running the expensive amount parsing.
        let consumed = self.amount_parser.parse_items_filtered(items, |item| {
            let in_range = item.x1 >= x1_range.0
                && item.x1 <= x1_range.1
                && item.x2 >= x2_range.0
                && item.x2 <= x2_range.1;
            if in_range {
                return true;
            }
            has_inverted_column
                && item.x1 >= invert_x1_range.0
                && item.x1 <= invert_x1_range.1
                && item.x2 >= invert_x2_range.0
                && item.x2 <= invert_x2_range.1
        });
        if consumed == 0 {
            return 0; // No amount found
        }
        let item = self.amount_parser.text_item();
        let x1_ok = item.x1 >= self.x1_range[0] && item.x1 <= self.x1_range[1];
        let x2_ok = item.x2 >= self.x2_range[0] && item.x2 <= self.x2_range[1];
        if x1_ok && x2_ok {
            return consumed;
        }
        // Must be in the invert range, since the filter above only allowed that as an alternative
        self.amount_parser.invert();
        consumed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configured_amount_ranges_are_used() {
        let config = StatementConfig {
            transaction_amount_x1_range: [10, 20],
            transaction_amount_x2_range: [30, 40],
            transaction_amount_invert_x1_range: [50, 60],
            transaction_amount_invert_x2_range: [70, 80],
            ..Default::default()
        };
        let parser = TransactionAmountParser::new(&config);

        assert_eq!(parser.x1_range, vec![10, 20]);
        assert_eq!(parser.x2_range, vec![30, 40]);
        assert_eq!(parser.invert_x1_range, vec![50, 60]);
        assert_eq!(parser.invert_x2_range, vec![70, 80]);
    }
}

use crate::parsers::base::{AmountParser, ParserPrimer};
use crate::structs::{ProtoTransaction, StatementConfig, TextItem};

pub struct TransactionBalanceParser {
    pub primed: bool,
    balance_parser: AmountParser,
    header_primer: ParserPrimer,
    alignment: String,
    x1_range: Vec<i32>,
    x2_range: Vec<i32>,
    x_tol: i32,
    invert: bool,
}

impl TransactionBalanceParser {
    pub fn new(config: &StatementConfig) -> Self {
        let primer_terms: Vec<&str> = config
            .transaction_balance_headers
            .iter()
            .map(|s| s.as_str())
            .collect();
        let balance_formats: Vec<&str> = config
            .transaction_balance_formats
            .iter()
            .map(|s| s.as_str())
            .collect();
        let alignment = config.transaction_balance_alignment.clone();
        let x_tol = config.transaction_alignment_tol;
        let invert = config.transaction_balance_invert;
        Self {
            primed: false,
            balance_parser: AmountParser::new(balance_formats.as_slice()),
            header_primer: ParserPrimer::new(primer_terms.as_slice(), 1),
            alignment,
            x_tol,
            x1_range: config.transaction_balance_x1_range.to_vec(),
            x2_range: config.transaction_balance_x2_range.to_vec(),
            invert,
        }
    }

    pub fn parse_items(&mut self, items: &[TextItem], transaction: &mut ProtoTransaction) -> usize {
        // Try reading and setting bounds from header
        let header_consumed = self.try_parse_header(items);
        if header_consumed > 0 {
            return header_consumed;
        }

        // Parser must be primed before parsing balances
        if !self.primed {
            return 0;
        }

        // Try parsing balance
        let balance_consumed = self.try_parse_balance(items);
        if balance_consumed > 0 {
            let mut value = self.balance_parser.value.unwrap();
            if self.invert {
                value = -value;
            }
            transaction.balance = Some(value);
            return balance_consumed;
        }
        0
    }

    /// Reset the parser state
    pub fn reset(&mut self) {
        self.primed = false;
        self.balance_parser.reset();
    }

    /// Set parser as primed
    pub fn prime(&mut self) {
        self.primed = true;
    }

    /// Get the maximum lookahead for the parser
    pub fn get_max_lookahead(&self) -> usize {
        let mut max_lookahead = 0;
        max_lookahead = max_lookahead.max(self.header_primer.max_lookahead);
        max_lookahead = max_lookahead.max(self.balance_parser.max_lookahead);
        max_lookahead
    }

    /// Check if header is set
    pub fn is_header_set(&self) -> bool {
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

    /// Try parsing balance and check if in x_ranges
    fn try_parse_balance(&mut self, items: &[TextItem]) -> usize {
        let x1_range = (self.x1_range[0], self.x1_range[1]);
        let x2_range = (self.x2_range[0], self.x2_range[1]);
        self.balance_parser.parse_items_filtered(items, |item| {
            item.x1 >= x1_range.0
                && item.x1 <= x1_range.1
                && item.x2 >= x2_range.0
                && item.x2 <= x2_range.1
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configured_ranges_are_used() {
        let config = StatementConfig {
            transaction_balance_x1_range: [10, 20],
            transaction_balance_x2_range: [30, 40],
            ..Default::default()
        };
        let parser = TransactionBalanceParser::new(&config);

        assert_eq!(parser.x1_range, vec![10, 20]);
        assert_eq!(parser.x2_range, vec![30, 40]);
    }
}

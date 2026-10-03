use crate::parsers::base::ParserPrimer;
use crate::structs::{ProtoTransaction, StatementConfig, TextItem};

pub struct TransactionDescriptionParser {
    pub primed: bool,
    header_primer: ParserPrimer,
    alignment: String,
    x1_range: Vec<i32>,
    x2_range: Vec<i32>,
    x_tol: i32,
    stray_description: String,
}

impl TransactionDescriptionParser {
    pub fn new(config: &StatementConfig) -> Self {
        let primer_terms: Vec<&str> = config
            .transaction_description_headers
            .iter()
            .map(|s| s.as_str())
            .collect();
        let alignment = config.transaction_description_alignment.clone();
        let x_tol = config.transaction_alignment_tol;
        Self {
            primed: false,
            header_primer: ParserPrimer::new(primer_terms.as_slice(), 1),
            alignment,
            x_tol,
            x1_range: config.transaction_description_x1_range.to_vec(),
            x2_range: config.transaction_description_x2_range.to_vec(),
            stray_description: String::new(),
        }
    }

    pub fn parse_items(&mut self, items: &[TextItem], transaction: &mut ProtoTransaction) -> usize {
        // Try reading and setting bounds from header
        let header_consumed = self.try_parse_header(items);
        if header_consumed > 0 {
            return header_consumed;
        }

        // Parser must be primed before parsing descriptions
        if !self.primed {
            return 0;
        }

        // Try parsing description
        let description_consumed = self.try_parse_description(items);
        if description_consumed > 0 {
            // Append text of first item to description
            let mut description = transaction.description.clone();
            if !description.is_empty() {
                description.push(' ');
            }
            description.push_str(&items[0].text);
            transaction.description = description;
            return description_consumed;
        }
        0
    }

    /// Reset the parser state
    pub fn reset(&mut self) {
        self.primed = false;
    }

    /// Set parser as primed
    pub fn prime(&mut self) {
        self.primed = true;
    }

    /// Check if header is set
    pub fn is_header_set(&self) -> bool {
        self.header_primer.primed
    }

    /// Get the maximum lookahead for the parser
    pub fn get_max_lookahead(&self) -> usize {
        let mut max_lookahead = 0;
        max_lookahead = max_lookahead.max(self.header_primer.max_lookahead);
        max_lookahead
    }

    /// Get the current x1/x2 bounds (e.g. to mirror onto another instance)
    pub fn bounds(&self) -> (Vec<i32>, Vec<i32>) {
        (self.x1_range.clone(), self.x2_range.clone())
    }

    /// Set the x1/x2 bounds directly (e.g. to mirror from another instance)
    pub fn set_bounds(&mut self, x1_range: Vec<i32>, x2_range: Vec<i32>) {
        self.x1_range = x1_range;
        self.x2_range = x2_range;
    }

    /// Try parsing a stray description fragment that falls within description
    /// bounds without requiring the parser to be primed. Used to catch
    /// out-of-band description text in vertically centered transactions.
    pub fn parse_stray(&mut self, items: &[TextItem]) -> usize {
        let consumed = self.try_parse_description(items);
        if consumed > 0 {
            if !self.stray_description.is_empty() {
                self.stray_description.push(' ');
            }
            self.stray_description.push_str(&items[0].text);
        }
        consumed
    }

    /// Take and clear the buffered stray description text
    pub fn take_stray(&mut self) -> Option<String> {
        if self.stray_description.is_empty() {
            return None;
        }
        Some(std::mem::take(&mut self.stray_description))
    }

    /// Adjust x1 or x2 bounds based on lowest/highest x positions
    pub fn adjust_bounds(&mut self, x_lowest: i32, x_highest: i32) {
        // Adjust upper bound if x_lowest is greater than first
        // bound and less than current upper bound
        if self.alignment == "x1" {
            if x_lowest > self.x1_range[0] && x_lowest < self.x1_range[1] {
                self.x1_range[1] = x_lowest + self.x_tol;
            }
        // Adjust lower bound if x_highest is less than second
        // bound and greater than current lower bound
        } else if self.alignment == "x2"
            && x_highest < self.x2_range[1]
            && x_highest > self.x2_range[0]
        {
            self.x2_range[0] = x_highest - self.x_tol;
        }
    }

    /// Try reading header and define x1 of x2 bounds
    fn try_parse_header(&mut self, items: &[TextItem]) -> usize {
        // Return if header already read
        if self.header_primer.primed {
            return 0;
        }
        let header_consumed = self.header_primer.parse_items(items);
        if header_consumed > 0 {
            let item = self.header_primer.text_item.as_ref().unwrap();
            if self.alignment == "x1" {
                self.x1_range = vec![item.x1 - self.x_tol, 10000];
            } else if self.alignment == "x2" {
                self.x2_range = vec![0, item.x2 + self.x_tol];
            }
        }
        header_consumed
    }

    /// Try parsing description - x1 and x2 of first item must be within ranges
    fn try_parse_description(&mut self, items: &[TextItem]) -> usize {
        if items.is_empty() {
            return 0;
        }
        let item = &items[0];
        if item.x1 >= self.x1_range[0]
            && item.x1 <= self.x1_range[1]
            && item.x2 >= self.x2_range[0]
            && item.x2 <= self.x2_range[1]
        {
            return 1; // Consumed 1 item
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configured_ranges_are_used_without_headers() {
        let config = StatementConfig {
            transaction_description_x1_range: [10, 20],
            transaction_description_x2_range: [30, 40],
            ..Default::default()
        };
        let parser = TransactionDescriptionParser::new(&config);

        assert_eq!(parser.x1_range, vec![10, 20]);
        assert_eq!(parser.x2_range, vec![30, 40]);
    }
}

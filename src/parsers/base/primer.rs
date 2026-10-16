use crate::structs::TextItem;
use crate::structs::benchmark::Timer;

/// A parser that is primed by matching terms from text items.
pub struct ParserPrimer {
    /// Parser is ready to scan for terms
    pub primed: bool,
    /// The last successfully parsed text item
    pub text_item: Option<TextItem>,
    /// The set of terms to match against (lowercase)
    pub terms: Vec<String>,
    /// Distinct supported whitespace-term counts, largest first
    item_counts: Vec<usize>,
    /// Number of space-delimited items in the longest term
    pub max_lookahead: usize,
    /// Number of times a term must be encountered before the parser is considered primed
    pub trigger_count: usize,
}

impl ParserPrimer {
    /// Create a new ParserPrimer with specified terms
    pub fn new(terms: &[&str], trigger_count: usize) -> Self {
        let terms_vec: Vec<String> = terms.iter().map(|t| t.to_string()).collect();
        let mut item_counts = terms_vec
            .iter()
            .map(|term| term.split_whitespace().count())
            .filter(|count| *count > 0)
            .collect::<Vec<_>>();
        item_counts.sort_unstable_by(|left, right| right.cmp(left));
        item_counts.dedup();
        let max_lookahead = terms_vec
            .iter()
            .map(|term| term.split_whitespace().count())
            .max()
            .unwrap_or(0);
        let mut parser = ParserPrimer {
            primed: false,
            text_item: None,
            terms: terms_vec,
            item_counts,
            max_lookahead,
            trigger_count,
        };
        if trigger_count == 0 {
            parser.primed = true;
        }
        parser
    }

    /// Get text item, raise error if none
    pub fn text_item(&self) -> &TextItem {
        self.text_item.as_ref().expect("No text item available")
    }

    /// Iteratively join text items and attempt to match terms (case sensitive)
    /// Returns number of items consumed if successful, else 0
    pub fn parse_items(&mut self, items: &[TextItem]) -> usize {
        if items.is_empty() {
            return 0;
        }
        if self.primed {
            return 0; // Already primed
        }
        let Some((consumed, text_item)) = self.find_match(items) else {
            return 0;
        };
        self.text_item = Some(text_item);
        self.trigger_count -= 1;
        if self.trigger_count == 0 {
            self.primed = true;
        }
        consumed
    }

    /// Return the number of leading items matching a primer term without
    /// changing primer state.
    pub fn matching_items(&self, items: &[TextItem]) -> usize {
        self.find_match(items)
            .map(|(consumed, _)| consumed)
            .unwrap_or(0)
    }

    fn find_match(&self, items: &[TextItem]) -> Option<(usize, TextItem)> {
        // Try longest first, then shorter.
        for term_count in &self.item_counts {
            let Some(consumed) = items_for_term_count(items, *term_count) else {
                continue;
            };
            if let Some(text_item) = TextItem::from_items(&items[0..consumed])
                && self.terms.iter().any(|term| term == &text_item.text)
            {
                return Some((consumed, text_item));
            }
        }
        None
    }

    pub fn parse_items_timed(&mut self, items: &[TextItem], timer: &mut Timer) -> usize {
        timer.start();
        let consumed = self.parse_items(items);
        timer.pause();
        consumed
    }

    /// Reset the parser
    pub fn reset(&mut self) {
        self.primed = false;
        self.text_item = None;
    }
}

fn items_for_term_count(items: &[TextItem], term_count: usize) -> Option<usize> {
    let mut accumulated_terms = 0;
    for (index, item) in items.iter().enumerate() {
        accumulated_terms += item.text.split_whitespace().count();
        if accumulated_terms == term_count {
            return Some(index + 1);
        }
        if accumulated_terms > term_count {
            return None;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::TextItem;

    fn make_text_item(text: &str) -> TextItem {
        TextItem {
            text: text.to_string(),
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
            page: 1,
            y1_bin: 0,
        }
    }

    #[test]
    fn test_single_term_match() {
        let mut parser = ParserPrimer::new(&["hello"], 1);
        let items = vec![make_text_item("hello")];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 1);
        assert!(parser.primed);
        assert_eq!(parser.text_item.as_ref().unwrap().text, "hello");
    }

    #[test]
    fn test_item_counts_are_distinct_and_descending() {
        let parser = ParserPrimer::new(&["hello", "good morning", "very good morning"], 1);
        assert_eq!(parser.item_counts, vec![3, 2, 1]);
    }

    #[test]
    fn test_multi_word_term_match() {
        let mut parser = ParserPrimer::new(&["hello world"], 1);
        let items = vec![make_text_item("hello"), make_text_item("world")];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 2);
        assert!(parser.primed);
        assert_eq!(parser.text_item.as_ref().unwrap().text, "hello world");
    }

    #[test]
    fn test_no_match() {
        let mut parser = ParserPrimer::new(&["foo"], 1);
        let items = vec![make_text_item("bar")];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 0);
        assert!(!parser.primed);
        assert!(parser.text_item.is_none());
    }

    #[test]
    fn test_reset() {
        let mut parser = ParserPrimer::new(&["hello"], 1);
        let items = vec![make_text_item("hello")];
        parser.parse_items(&items);
        assert!(parser.primed);
        parser.primed = false;
        parser.text_item = None;
        assert!(!parser.primed);
        assert!(parser.text_item.is_none());
    }

    #[test]
    fn test_empty_items() {
        let mut parser = ParserPrimer::new(&["hello"], 1);
        let items: Vec<TextItem> = vec![];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 0);
        assert!(!parser.primed);
        assert!(parser.text_item.is_none());
    }
}

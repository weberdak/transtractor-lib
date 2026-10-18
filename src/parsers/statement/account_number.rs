use crate::parsers::base::{ParserPrimer, ValueParser};
use crate::parsers::primed::PrimedValueParser;
use crate::structs::benchmark::Timer;
use crate::structs::{AccountData, StatementConfig, TextItem};

/// State for statements where the account number precedes its identifying terms.
struct TermsAfterState {
    primer: ParserPrimer,
    value_parser: ValueParser,
    /// Latest matching account number, held until a term aligns with it
    buffer: Option<TextItem>,
    alignment: String,
    alignment_tol: i32,
}

impl TermsAfterState {
    fn is_aligned(&self, value: &TextItem, term: &TextItem) -> bool {
        let tol = self.alignment_tol;
        let aligned = match self.alignment.as_str() {
            "x1" => (value.x1 - term.x1).abs() <= tol,
            "x2" => (value.x2 - term.x2).abs() <= tol,
            "y1" => (value.y1 - term.y1).abs() <= tol,
            "y2" => (value.y2 - term.y2).abs() <= tol,
            _ => true,
        };
        aligned && value.page == term.page
    }
}

pub struct AccountNumberParser {
    pub(crate) parser: PrimedValueParser,
    after: Option<TermsAfterState>,
    found: Option<String>,
}

impl AccountNumberParser {
    pub fn new(config: &StatementConfig) -> Self {
        let primer_terms: Vec<&str> = config
            .account_number_terms
            .iter()
            .map(|s| s.as_str())
            .collect();
        let value_patterns: Vec<regex::Regex> = config
            .account_number_patterns
            .iter()
            .filter_map(|p| regex::Regex::new(p.as_str()).ok())
            .collect();
        let after = config.account_number_terms_after.then(|| TermsAfterState {
            primer: ParserPrimer::new(primer_terms.as_slice(), 1),
            value_parser: ValueParser::new(value_patterns.as_slice()),
            buffer: None,
            alignment: config.account_number_alignment.clone(),
            alignment_tol: config.account_number_alignment_tol,
        });
        Self {
            parser: PrimedValueParser::new(
                primer_terms.as_slice(),
                value_patterns.as_slice(),
                config.account_number_alignment.as_str(),
                config.account_number_alignment_tol,
                config.account_number_trigger_count,
            ),
            after,
            found: None,
        }
    }

    pub fn parse_items(&mut self, items: &[TextItem], data: &mut AccountData) -> usize {
        self.parse_items_inner(items, data, None)
    }

    pub fn parse_items_timed(
        &mut self,
        items: &[TextItem],
        data: &mut AccountData,
        prime_timer: &mut Timer,
    ) -> usize {
        self.parse_items_inner(items, data, Some(prime_timer))
    }

    fn parse_items_inner(
        &mut self,
        items: &[TextItem],
        data: &mut AccountData,
        prime_timer: Option<&mut Timer>,
    ) -> usize {
        if self.after.is_some() {
            return self.parse_terms_after(items, data, prime_timer);
        }
        let consumed = match prime_timer {
            Some(timer) => self.parser.parse_items_timed(items, timer),
            None => self.parser.parse_items(items),
        };
        self.update_data(consumed, data);
        consumed
    }

    /// Buffer each matching account number; accept it once an aligned term follows.
    fn parse_terms_after(
        &mut self,
        items: &[TextItem],
        data: &mut AccountData,
        mut prime_timer: Option<&mut Timer>,
    ) -> usize {
        let Some(state) = self.after.as_mut() else {
            return 0;
        };
        if items.is_empty() || self.found.is_some() {
            return 0;
        }
        if let Some(timer) = prime_timer.as_deref_mut() {
            timer.start();
        }
        let term_len = state.primer.matching_items(items);
        let term = (term_len > 0)
            .then(|| TextItem::from_items(&items[..term_len]))
            .flatten();
        if let Some(timer) = prime_timer {
            timer.pause();
        }

        // Terms are not consumed so other parsers can still see them
        if let Some(term) = term {
            if let Some(value) = &state.buffer
                && state.is_aligned(value, &term)
            {
                self.found = Some(value.text.clone());
                if data.account_number().is_none() {
                    data.set_account_number(value.text.clone());
                }
            }
            return 0;
        }

        let consumed = state.value_parser.parse_items(items);
        if consumed > 0 {
            state.buffer = state.value_parser.text_item.take();
            state.value_parser.reset();
        }
        consumed
    }

    fn update_data(&self, consumed: usize, data: &mut AccountData) {
        if consumed > 0
            && data.account_number().is_none()
            && let Some(value) = self.parser.value()
        {
            data.set_account_number(value.to_string());
        }
    }

    pub fn get_max_lookahead(&self) -> usize {
        match &self.after {
            Some(state) => state
                .primer
                .max_lookahead
                .max(state.value_parser.max_lookahead),
            None => self.parser.get_max_lookahead(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::{AccountData, StatementConfig, TextItem};

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

    fn default_config() -> StatementConfig {
        StatementConfig {
            account_number_terms: vec!["Account Number".to_string()],
            account_number_patterns: vec![regex::Regex::new(r"\b\d+\s+\d+\s+\d+\b").unwrap()],
            account_number_alignment: "x1".to_string(),
            account_number_alignment_tol: 5,
            ..Default::default()
        }
    }

    #[test]
    fn test_account_number_success() {
        let config = default_config();
        let mut data = AccountData::new();
        let mut parser = AccountNumberParser::new(&config);

        let items = vec![
            make_text_item("Account Number", 100, 200, 1),
            make_text_item("1234", 102, 202, 1),
            make_text_item("5678", 152, 202, 1),
            make_text_item("9012", 202, 202, 1),
        ];

        // Parse primer
        let consumed_primer = parser.parse_items(&items, &mut data);
        assert_eq!(consumed_primer, 0);
        assert!(parser.parser.is_primed());
        assert!(data.account_number().is_none());

        // Parse account number
        let consumed_account = parser.parse_items(&items[1..], &mut data);
        assert_eq!(consumed_account, 3);
        assert_eq!(parser.parser.value(), Some("1234 5678 9012"));
        assert_eq!(data.account_number(), Some(&"1234 5678 9012".to_string()));
    }

    #[test]
    fn test_account_number_without_primer() {
        let config = default_config();
        let mut data = AccountData::new();
        let mut parser = AccountNumberParser::new(&config);

        let items = vec![
            make_text_item("1234", 100, 200, 1),
            make_text_item("5678", 150, 200, 1),
            make_text_item("9012", 200, 200, 1),
        ];

        let consumed = parser.parse_items(&items, &mut data);
        assert_eq!(consumed, 0);
        assert!(data.account_number().is_none());
    }

    #[test]
    fn test_account_number_no_match() {
        let config = default_config();
        let mut data = AccountData::new();
        let mut parser = AccountNumberParser::new(&config);

        let items = vec![
            make_text_item("Account Number", 100, 200, 1),
            make_text_item("INVALID", 102, 202, 1),
        ];

        parser.parse_items(&items, &mut data);
        let consumed = parser.parse_items(&items[1..], &mut data);
        assert_eq!(consumed, 0);
        assert!(data.account_number().is_none());
    }

    #[test]
    fn test_account_number_x1_constraint_fail() {
        let config = default_config();
        let mut data = AccountData::new();
        let mut parser = AccountNumberParser::new(&config);

        let items = vec![
            make_text_item("Account Number", 100, 200, 1),
            make_text_item("1234", 150, 202, 1), // x1 too far (50 > tolerance of 5)
            make_text_item("5678", 200, 202, 1),
            make_text_item("9012", 250, 202, 1),
        ];

        parser.parse_items(&items, &mut data);
        let consumed = parser.parse_items(&items[1..], &mut data);
        assert_eq!(consumed, 0);
        assert!(data.account_number().is_none());
    }

    #[test]
    fn test_account_number_y1_constraint_fail() {
        let mut config = default_config();
        config.account_number_alignment = "y1".to_string();
        let mut data = AccountData::new();
        let mut parser = AccountNumberParser::new(&config);

        let items = vec![
            make_text_item("Account Number", 100, 200, 1),
            make_text_item("1234", 102, 250, 1), // y1 too far (50 > tolerance of 5)
            make_text_item("5678", 152, 250, 1),
            make_text_item("9012", 202, 250, 1),
        ];

        parser.parse_items(&items, &mut data);
        let consumed = parser.parse_items(&items[1..], &mut data);
        assert_eq!(consumed, 0);
        assert!(data.account_number().is_none());
    }

    #[test]
    fn test_account_number_page_constraint_fail() {
        let config = default_config();
        let mut data = AccountData::new();
        let mut parser = AccountNumberParser::new(&config);

        let items = vec![
            make_text_item("Account Number", 100, 200, 1),
            make_text_item("1234", 102, 202, 2), // Different page
            make_text_item("5678", 152, 202, 2),
            make_text_item("9012", 202, 202, 2),
        ];

        parser.parse_items(&items, &mut data);
        let consumed = parser.parse_items(&items[1..], &mut data);
        assert_eq!(consumed, 0);
        assert!(data.account_number().is_none());
    }

    #[test]
    fn test_account_number_already_set() {
        let config = default_config();
        let mut data = AccountData::new();
        data.set_account_number("9999 8888 7777".to_string());
        let mut parser = AccountNumberParser::new(&config);

        let items = vec![
            make_text_item("Account Number", 100, 200, 1),
            make_text_item("1234", 102, 202, 1),
            make_text_item("5678", 152, 202, 1),
            make_text_item("9012", 202, 202, 1),
        ];

        parser.parse_items(&items, &mut data);
        parser.parse_items(&items[1..], &mut data);

        // Should keep original value
        assert_eq!(data.account_number(), Some(&"9999 8888 7777".to_string()));
    }

    #[test]
    fn test_account_number_parser_already_parsed() {
        let config = default_config();
        let mut data = AccountData::new();
        let mut parser = AccountNumberParser::new(&config);

        let items1 = vec![
            make_text_item("Account Number", 100, 200, 1),
            make_text_item("1234", 102, 202, 1),
            make_text_item("5678", 152, 202, 1),
            make_text_item("9012", 202, 202, 1),
        ];

        parser.parse_items(&items1, &mut data);
        parser.parse_items(&items1[1..], &mut data);
        assert_eq!(data.account_number(), Some(&"1234 5678 9012".to_string()));

        // Try parsing different account number
        let items2 = vec![
            make_text_item("Account Number", 100, 300, 1),
            make_text_item("5555", 102, 302, 1),
            make_text_item("6666", 152, 302, 1),
            make_text_item("7777", 202, 302, 1),
        ];

        let consumed = parser.parse_items(&items2, &mut data);
        assert_eq!(consumed, 0); // Parser already has value, returns 0
        assert_eq!(data.account_number(), Some(&"1234 5678 9012".to_string())); // Original value
    }

    #[test]
    fn test_account_number_max_lookahead() {
        let config = default_config();
        let parser = AccountNumberParser::new(&config);

        // Pattern "\b\d+\s+\d+\s+\d+\b" has 2 \s separators = 3 tokens
        assert_eq!(parser.get_max_lookahead(), 3);
    }

    #[test]
    fn test_account_number_single_token() {
        let mut config = default_config();
        config.account_number_patterns = vec![regex::Regex::new(r"\b\d{4}\b").unwrap()];

        let mut data = AccountData::new();
        let mut parser = AccountNumberParser::new(&config);

        let items = vec![
            make_text_item("Account Number", 100, 200, 1),
            make_text_item("1234", 102, 202, 1),
        ];

        parser.parse_items(&items, &mut data);
        let consumed = parser.parse_items(&items[1..], &mut data);

        assert_eq!(consumed, 1);
        assert_eq!(data.account_number(), Some(&"1234".to_string()));
    }

    #[test]
    fn test_account_number_multiple_patterns() {
        let mut config = default_config();
        config.account_number_patterns = vec![
            regex::Regex::new(r"\b\d+-\d+-\d+\b").unwrap(),
            regex::Regex::new(r"\b\d+\s+\d+\s+\d+\b").unwrap(),
        ];

        let mut data = AccountData::new();
        let mut parser = AccountNumberParser::new(&config);

        // Test hyphen-separated pattern
        let items = vec![
            make_text_item("Account Number", 100, 200, 1),
            make_text_item("1234-5678-9012", 102, 202, 1),
        ];

        parser.parse_items(&items, &mut data);
        let consumed = parser.parse_items(&items[1..], &mut data);

        assert_eq!(consumed, 1);
        assert_eq!(data.account_number(), Some(&"1234-5678-9012".to_string()));
    }

    #[test]
    fn test_account_number_no_x1_constraint() {
        let mut config = default_config();
        config.account_number_alignment = "".to_string();

        let mut data = AccountData::new();
        let mut parser = AccountNumberParser::new(&config);

        let items = vec![
            make_text_item("Account Number", 100, 200, 1),
            make_text_item("1234", 500, 202, 1), // Far x1, but constraint disabled
            make_text_item("5678", 550, 202, 1),
            make_text_item("9012", 600, 202, 1),
        ];

        parser.parse_items(&items, &mut data);
        let consumed = parser.parse_items(&items[1..], &mut data);

        assert_eq!(consumed, 3);
        assert_eq!(data.account_number(), Some(&"1234 5678 9012".to_string()));
    }

    #[test]
    fn test_account_number_no_y1_constraint() {
        let mut config = default_config();
        config.account_number_alignment = "".to_string();

        let mut data = AccountData::new();
        let mut parser = AccountNumberParser::new(&config);

        let items = vec![
            make_text_item("Account Number", 100, 200, 1),
            make_text_item("1234", 102, 500, 1), // Far y1, but constraint disabled
            make_text_item("5678", 152, 500, 1),
            make_text_item("9012", 202, 500, 1),
        ];

        parser.parse_items(&items, &mut data);
        let consumed = parser.parse_items(&items[1..], &mut data);

        assert_eq!(consumed, 3);
        assert_eq!(data.account_number(), Some(&"1234 5678 9012".to_string()));
    }

    #[test]
    fn test_terms_after_buffers_value_until_term() {
        let config = StatementConfig {
            account_number_terms: vec!["DEFBANK CARD".to_string()],
            account_number_patterns: vec![regex::Regex::new(r"^\d{8}$").unwrap()],
            account_number_alignment: "y1".to_string(),
            account_number_alignment_tol: 5,
            account_number_terms_after: true,
            ..Default::default()
        };
        let mut data = AccountData::new();
        let mut parser = AccountNumberParser::new(&config);
        let items = vec![
            make_text_item("11111111", 10, 100, 1),
            make_text_item("12345678", 10, 200, 1),
            make_text_item("DEFBANK", 100, 202, 1),
            make_text_item("CARD", 150, 202, 1),
        ];
        assert_eq!(parser.parse_items(&items, &mut data), 1);
        assert_eq!(parser.parse_items(&items[1..], &mut data), 1);
        assert!(data.account_number().is_none());
        assert_eq!(parser.parse_items(&items[2..], &mut data), 0);
        assert_eq!(data.account_number().map(String::as_str), Some("12345678"));
    }

    #[test]
    fn test_terms_after_misaligned_term_ignored() {
        let config = StatementConfig {
            account_number_terms: vec!["CARD".to_string()],
            account_number_patterns: vec![regex::Regex::new(r"^\d{8}$").unwrap()],
            account_number_alignment: "y1".to_string(),
            account_number_alignment_tol: 5,
            account_number_terms_after: true,
            ..Default::default()
        };
        let mut data = AccountData::new();
        let mut parser = AccountNumberParser::new(&config);
        let items = vec![
            make_text_item("12345678", 10, 100, 1),
            make_text_item("CARD", 100, 300, 1),
        ];
        parser.parse_items(&items, &mut data);
        parser.parse_items(&items[1..], &mut data);
        assert!(data.account_number().is_none());
    }
}

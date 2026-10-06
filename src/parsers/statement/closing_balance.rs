use crate::parsers::primed::PrimedAmountParser;
use crate::structs::benchmark::Timer;
use crate::structs::{AccountData, StatementConfig, TextItem};

pub struct ClosingBalanceParser {
    pub(crate) parser: PrimedAmountParser,
}

impl ClosingBalanceParser {
    pub fn new(config: &StatementConfig) -> Self {
        let primer_terms: Vec<&str> = config
            .closing_balance_terms
            .iter()
            .map(|s| s.as_str())
            .collect();
        let amount_formats: Vec<&str> = config
            .closing_balance_formats
            .iter()
            .map(|s| s.as_str())
            .collect();
        Self {
            parser: PrimedAmountParser::new(
                primer_terms.as_slice(),
                amount_formats.as_slice(),
                &config.closing_balance_alignment,
                config.closing_balance_alignment_tol,
                config.closing_balance_invert,
                config.closing_balance_trigger_count,
            ),
        }
    }

    pub fn parse_items(&mut self, items: &[TextItem], data: &mut AccountData) -> usize {
        let consumed = self.parser.parse_items(items);
        self.update_data(consumed, data);
        consumed
    }

    pub fn parse_items_timed(
        &mut self,
        items: &[TextItem],
        data: &mut AccountData,
        prime_timer: &mut Timer,
    ) -> usize {
        let consumed = self.parser.parse_items_timed(items, prime_timer);
        self.update_data(consumed, data);
        consumed
    }

    fn update_data(&self, consumed: usize, data: &mut AccountData) {
        if consumed > 0
            && let Some(value) = self.parser.value()
            && data.closing_balance().is_none()
        {
            data.set_closing_balance(value);
        }
    }

    pub fn get_max_lookahead(&self) -> usize {
        self.parser.get_max_lookahead()
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
            closing_balance_terms: vec!["CLOSING BALANCE".to_string()],
            closing_balance_formats: vec!["format1".to_string()],
            closing_balance_alignment: "x1".to_string(),
            closing_balance_alignment_tol: 5,
            closing_balance_invert: false,
            ..Default::default()
        }
    }

    #[test]
    fn test_closing_balance_success() {
        let config = default_config();
        let mut data = AccountData::new();
        let mut parser = ClosingBalanceParser::new(&config);

        let items = vec![
            make_text_item("CLOSING BALANCE", 100, 200, 1),
            make_text_item("9,876.54", 102, 202, 1),
        ];

        let consumed_primer = parser.parse_items(&items, &mut data);
        assert_eq!(consumed_primer, 1);
        assert!(parser.parser.is_primed());
        assert!(parser.parser.value().is_none());

        let consumed_amount = parser.parse_items(&items[1..], &mut data);
        assert_eq!(consumed_amount, 1);
        assert_eq!(parser.parser.value(), Some(9876.54));
        assert_eq!(data.closing_balance(), Some(9876.54));
    }

    #[test]
    fn test_closing_balance_invert() {
        let mut config = default_config();
        config.closing_balance_invert = true;
        let mut data = AccountData::new();
        let mut parser = ClosingBalanceParser::new(&config);

        let items = vec![
            make_text_item("CLOSING BALANCE", 100, 200, 1),
            make_text_item("9,876.54", 100, 200, 1),
        ];

        parser.parse_items(&items, &mut data);
        let consumed = parser.parse_items(&items[1..], &mut data);
        assert_eq!(consumed, 1);
        assert_eq!(parser.parser.value(), Some(-9876.54));
        assert_eq!(data.closing_balance(), Some(-9876.54));
    }

    #[test]
    fn test_closing_balance_fail() {
        let config = default_config();
        let mut data = AccountData::new();
        let mut parser = ClosingBalanceParser::new(&config);

        let items = vec![
            make_text_item("NOT CLOSING BALANCE", 100, 200, 1),
            make_text_item("9,876.54", 102, 202, 1),
        ];

        let consumed = parser.parse_items(&items, &mut data);
        assert_eq!(consumed, 0);
        assert!(data.closing_balance().is_none());
    }

    #[test]
    fn test_closing_balance_page_fail() {
        let config = default_config();
        let mut data = AccountData::new();
        let mut parser = ClosingBalanceParser::new(&config);

        let items = vec![
            make_text_item("CLOSING BALANCE", 100, 200, 1),
            make_text_item("9,876.54", 100, 200, 2),
        ];

        parser.parse_items(&items, &mut data);
        let consumed = parser.parse_items(&items[1..], &mut data);
        assert_eq!(consumed, 0);
        assert!(data.closing_balance().is_none());
    }
}

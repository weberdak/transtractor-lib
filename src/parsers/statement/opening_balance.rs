use crate::parsers::primed::PrimedAmountParser;
use crate::structs::benchmark::Timer;
use crate::structs::{AccountData, StatementConfig, TextItem};

pub struct OpeningBalanceParser {
    parser: PrimedAmountParser,
}

impl OpeningBalanceParser {
    pub fn new(config: &StatementConfig) -> Self {
        // Convert Vec<String> to Vec<&str> for constructor expectations
        let primer_terms: Vec<&str> = config
            .opening_balance_terms
            .iter()
            .map(|s| s.as_str())
            .collect();
        let amount_formats: Vec<&str> = config
            .opening_balance_formats
            .iter()
            .map(|s| s.as_str())
            .collect();
        Self {
            parser: PrimedAmountParser::new(
                primer_terms.as_slice(),
                amount_formats.as_slice(),
                &config.opening_balance_alignment,
                config.opening_balance_alignment_tol,
                config.opening_balance_invert,
                config.opening_balance_trigger_count,
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
        if consumed > 0 && self.parser.value().is_some() && data.opening_balance().is_none() {
            // Only set if not already set to avoid overwriting a prior successful parse
            if let Some(value) = self.parser.value() {
                data.set_opening_balance(value);
            }
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
            opening_balance_terms: vec!["OPENING BALANCE".to_string()],
            opening_balance_formats: vec!["format1".to_string()],
            opening_balance_alignment: "x1".to_string(),
            opening_balance_alignment_tol: 5,
            opening_balance_invert: false,
            // Add other fields as needed, or use StatementConfig::default() if available
            ..Default::default()
        }
    }

    #[test]
    fn test_opening_balance_success() {
        let config = default_config();
        let mut data = AccountData::new();
        let mut parser = OpeningBalanceParser::new(&config);

        let items = vec![
            make_text_item("OPENING BALANCE", 100, 200, 1),
            make_text_item("1,234.56", 102, 202, 1),
        ];

        let consumed_primer = parser.parse_items(&items, &mut data);
        assert_eq!(consumed_primer, 1);
        assert!(parser.parser.is_primed());
        assert!(parser.parser.value().is_none());

        let consumed_amount = parser.parse_items(&items[1..], &mut data);
        assert_eq!(consumed_amount, 1);
        assert_eq!(parser.parser.value(), Some(1234.56));
        assert_eq!(data.opening_balance(), Some(1234.56));
    }

    #[test]
    fn test_opening_balance_invert() {
        let mut config = default_config();
        config.opening_balance_invert = true;
        let mut data = AccountData::new();
        let mut parser = OpeningBalanceParser::new(&config);

        let items = vec![
            make_text_item("OPENING BALANCE", 100, 200, 1),
            make_text_item("1,234.56", 100, 200, 1),
        ];

        parser.parse_items(&items, &mut data);
        let consumed = parser.parse_items(&items[1..], &mut data);
        assert_eq!(consumed, 1);
        assert_eq!(parser.parser.value(), Some(-1234.56));
        assert_eq!(data.opening_balance(), Some(-1234.56));
    }

    #[test]
    fn test_opening_balance_fail() {
        let config = default_config();
        let mut data = AccountData::new();
        let mut parser = OpeningBalanceParser::new(&config);

        let items = vec![
            make_text_item("NOT OPENING BALANCE", 100, 200, 1),
            make_text_item("1,234.56", 102, 202, 1),
        ];

        let consumed = parser.parse_items(&items, &mut data);
        assert_eq!(consumed, 0);
        assert!(data.opening_balance().is_none());
    }

    #[test]
    fn test_opening_balance_page_fail() {
        let config = default_config();
        let mut data = AccountData::new();
        let mut parser = OpeningBalanceParser::new(&config);

        let items = vec![
            make_text_item("OPENING BALANCE", 100, 200, 1),
            make_text_item("1,234.56", 100, 200, 2),
        ];

        parser.parse_items(&items, &mut data);
        let consumed = parser.parse_items(&items[1..], &mut data);
        assert_eq!(consumed, 0);
        assert!(data.opening_balance().is_none());
    }
}

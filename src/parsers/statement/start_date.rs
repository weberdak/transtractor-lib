use crate::parsers::primed::PrimedDateParser;
use crate::structs::benchmark::Timer;
use crate::structs::{AccountData, StatementConfig, TextItem};

pub struct StartDateParser {
    pub(crate) parser: PrimedDateParser,
}

impl StartDateParser {
    pub fn new(config: &StatementConfig) -> Self {
        let primer_terms: Vec<&str> = config.start_date_terms.iter().map(|s| s.as_str()).collect();
        let date_formats: Vec<&str> = config
            .start_date_formats
            .iter()
            .map(|s| s.as_str())
            .collect();
        Self {
            parser: PrimedDateParser::new(
                primer_terms.as_slice(),
                date_formats.as_slice(),
                &config.start_date_alignment,
                config.start_date_alignment_tol,
                config.start_date_trigger_count,
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
            && data.start_date().is_none()
        {
            data.set_start_date(value);
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
            start_date_terms: vec!["STATEMENT PERIOD".to_string(), "FROM".to_string()],
            start_date_formats: vec!["format2".to_string()],
            start_date_alignment: "x1".to_string(),
            start_date_alignment_tol: 5,
            ..Default::default()
        }
    }

    #[test]
    fn test_start_date_success() {
        let config = default_config();
        let mut data = AccountData::new();
        let mut parser = StartDateParser::new(&config);

        let items = vec![
            make_text_item("FROM", 100, 200, 1),
            make_text_item("24", 100, 200, 1),
            make_text_item("march", 100, 200, 1),
            make_text_item("2020", 100, 200, 1),
        ];

        let consumed_primer = parser.parse_items(&items, &mut data);
        assert_eq!(consumed_primer, 1);
        assert!(parser.parser.is_primed());
        assert!(parser.parser.value().is_none());

        let consumed_date = parser.parse_items(&items[1..], &mut data);
        assert_eq!(consumed_date, 3);
        assert!(parser.parser.value().is_some());
        assert!(data.start_date().is_some());
    }

    #[test]
    fn test_start_date_fail_no_match() {
        let config = default_config();
        let mut data = AccountData::new();
        let mut parser = StartDateParser::new(&config);

        let items = vec![
            make_text_item("NOT", 100, 200, 1),
            make_text_item("A", 100, 200, 1),
            make_text_item("DATE", 100, 200, 1),
        ];

        let consumed = parser.parse_items(&items, &mut data);
        assert_eq!(consumed, 0);
        assert!(data.start_date().is_none());
    }

    #[test]
    fn test_start_date_page_mismatch() {
        let config = default_config();
        let mut data = AccountData::new();
        let mut parser = StartDateParser::new(&config);

        let items = vec![
            make_text_item("FROM", 100, 200, 1),
            make_text_item("24", 100, 200, 2),
            make_text_item("march", 100, 200, 2),
            make_text_item("2020", 100, 200, 2),
        ];

        let consumed_primer = parser.parse_items(&items, &mut data);
        assert_eq!(consumed_primer, 1);
        let consumed_date = parser.parse_items(&items[1..], &mut data);
        assert_eq!(consumed_date, 0);
        assert!(data.start_date().is_none());
    }
}

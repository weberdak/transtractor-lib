use crate::parsers::base::ParserPrimer;
use crate::parsers::transaction;
use crate::parsers::transaction::{
    TransactionAmountParser, TransactionBalanceParser, TransactionDateParser,
    TransactionDescriptionParser,
};
use crate::structs::AccountData;
use crate::structs::ProtoTransaction;
use crate::structs::StatementConfig;
use crate::structs::TextItem;
use crate::structs::benchmark::Timer;
use regex::Regex;
use std::collections::HashMap;

pub struct TransactionParser {
    date_parser: TransactionDateParser,
    date_parser_newline: TransactionDateParser,
    start_date_required: bool,
    description_parser: TransactionDescriptionParser,
    description_parser_stray: TransactionDescriptionParser,
    amount_parser: TransactionAmountParser,
    amount_parser_newline: TransactionAmountParser,
    balance_parser: TransactionBalanceParser,
    balance_parser_newline: TransactionBalanceParser,
    start_primer: ParserPrimer,
    stop_primer: ParserPrimer,
    current_transaction: ProtoTransaction,
    compulsory_fields: Vec<String>,
    all_fields: Vec<String>,
    new_line_fields: Vec<String>,
    end_line_fields: Vec<String>,
    next_fields: HashMap<String, Vec<String>>,
    current_line_y1_bin: i32,
    description_x_bounds_adjusted: bool,
    description_exclude_patterns: Vec<Regex>,
}

impl TransactionParser {
    pub fn new(config: &StatementConfig) -> Self {
        let transaction_formats = config.transaction_formats.clone();
        let new_line_fields = transaction::utils::get_new_line_fields(transaction_formats.clone());
        let end_line_fields = transaction::utils::get_end_line_fields(transaction_formats.clone());
        let next_fields = transaction::utils::get_next_fields(transaction_formats.clone());
        let compulsory_fields =
            transaction::utils::get_compulsory_fields(transaction_formats.clone());
        let all_fields = transaction::utils::get_all_fields(transaction_formats);
        let start_terms: Vec<&str> = config
            .transaction_terms
            .iter()
            .map(|s| s.as_str())
            .collect();
        let stop_terms: Vec<&str> = config
            .transaction_terms_stop
            .iter()
            .map(|s| s.as_str())
            .collect();

        TransactionParser {
            date_parser: TransactionDateParser::new(config),
            date_parser_newline: TransactionDateParser::new(config),
            start_date_required: config.transaction_start_date_required,
            description_parser: TransactionDescriptionParser::new(config),
            description_parser_stray: TransactionDescriptionParser::new(config),
            amount_parser: TransactionAmountParser::new(config),
            amount_parser_newline: TransactionAmountParser::new(config),
            balance_parser: TransactionBalanceParser::new(config),
            balance_parser_newline: TransactionBalanceParser::new(config),
            start_primer: ParserPrimer::new(&start_terms, config.transaction_trigger_count),
            stop_primer: ParserPrimer::new(&stop_terms, config.transaction_stop_trigger_count),
            current_transaction: ProtoTransaction::new(),
            compulsory_fields,
            all_fields,
            new_line_fields,
            end_line_fields,
            next_fields,
            current_line_y1_bin: 0,
            description_x_bounds_adjusted: false,
            description_exclude_patterns: config.transaction_description_exclude.clone(),
        }
    }

    pub fn parse_items(&mut self, items: &[TextItem], data: &mut AccountData) -> usize {
        self.parse_items_inner(items, data, None, None)
    }

    pub fn parse_items_timed(
        &mut self,
        items: &[TextItem],
        data: &mut AccountData,
        start_prime_timer: &mut Timer,
        stop_prime_timer: &mut Timer,
    ) -> usize {
        self.parse_items_inner(items, data, Some(start_prime_timer), Some(stop_prime_timer))
    }

    fn parse_items_inner(
        &mut self,
        items: &[TextItem],
        data: &mut AccountData,
        start_prime_timer: Option<&mut Timer>,
        stop_prime_timer: Option<&mut Timer>,
    ) -> usize {
        // Handle/check for start/stop primers - these are not consumed
        let start_consumed = match start_prime_timer {
            Some(timer) => self.start_primer.parse_items_timed(items, timer),
            None => self.start_primer.parse_items(items),
        };
        if start_consumed > 0 {
            if self.start_date_required && data.start_date().is_none() {
                data.add_error(
                    "Statement config requires a start date is set prior to parsing transactions."
                        .to_string(),
                );
                return usize::MAX;
            }
            self.date_parser.set_start_date_year(data);
            self.date_parser_newline.set_start_date_year(data);
        }

        match stop_prime_timer {
            Some(timer) => self.stop_primer.parse_items_timed(items, timer),
            None => self.stop_primer.parse_items(items),
        };
        if !self.start_primer.primed || self.stop_primer.primed {
            if self.stop_primer.primed {
                self.merge_stray_into_last(data);
            }
            return 0;
        }

        // Adjust description parser x_bounds if needed
        self.adjust_description_x_bounds();
        self.sync_description_stray_bounds();

        // Handle new line, if one
        let is_new_line = self.is_new_line(items);
        self.current_line_y1_bin = items[0].y1_bin;
        if is_new_line {
            let consumed = self.handle_new_line(items, data);
            if consumed > 0 {
                return consumed;
            }
        }

        // Try parsing date
        let date_consumed = self
            .date_parser
            .parse_items(items, &mut self.current_transaction);
        if date_consumed > 0 {
            self.date_parser.reset();
            self.post_parse_append("date".to_string(), data);
            self.post_parse_prime("date".to_string());
            return date_consumed;
        }

        // Try parsing amount
        let amount_consumed = self
            .amount_parser
            .parse_items(items, &mut self.current_transaction);
        if amount_consumed > 0 {
            self.amount_parser.reset();
            self.post_parse_append("amount".to_string(), data);
            self.post_parse_prime("amount".to_string());
            return amount_consumed;
        }

        // Try parsing balance
        let balance_consumed = self
            .balance_parser
            .parse_items(items, &mut self.current_transaction);
        if balance_consumed > 0 {
            self.balance_parser.reset();
            self.post_parse_append("balance".to_string(), data);
            self.post_parse_prime("balance".to_string());
            return balance_consumed;
        }

        // Try parsing description
        let description_consumed = self
            .description_parser
            .parse_items(items, &mut self.current_transaction);
        if description_consumed > 0 {
            return description_consumed;
        }

        // Try capturing stray description text (e.g. from vertically centred
        // transactions) that falls within description bounds but was missed
        // because no field parser was primed to accept it on this line.
        // Skip this if amount/balance are primed: a failed match there means
        // genuinely malformed field data, not out-of-band description text.
        if !self.amount_parser.primed && !self.balance_parser.primed {
            let stray_consumed = self.description_parser_stray.parse_stray(items);
            if stray_consumed > 0 {
                return stray_consumed;
            }
        }
        0
    }

    /// Get the maximum lookahead for the parser
    pub fn get_max_lookahead(&self) -> usize {
        let mut max_lookahead = 0;
        max_lookahead = max_lookahead.max(self.start_primer.max_lookahead);
        max_lookahead = max_lookahead.max(self.stop_primer.max_lookahead);
        max_lookahead = max_lookahead.max(self.date_parser.get_max_lookahead());
        max_lookahead = max_lookahead.max(self.amount_parser.get_max_lookahead());
        max_lookahead = max_lookahead.max(self.balance_parser.get_max_lookahead());
        max_lookahead = max_lookahead.max(self.description_parser.get_max_lookahead());
        max_lookahead
    }

    /// Check if all compulsory field headers are set
    fn all_headers_set(&self) -> bool {
        for field in &self.all_fields {
            match field.as_str() {
                "date" if !self.date_parser.is_header_set() => return false,
                "description" if !self.description_parser.is_header_set() => return false,
                "amount" if !self.amount_parser.is_header_set() => return false,
                "balance" if !self.balance_parser.is_header_set() => return false,
                _ => {}
            }
        }
        true
    }

    /// Get effective x_bounds for a specified parser
    fn get_parser_x_bounds(&self, field: &str) -> (i32, i32) {
        match field {
            "date" => self.date_parser.get_x_bounds(),
            "amount" => self.amount_parser.get_x_bounds(),
            "balance" => self.balance_parser.get_x_bounds(),
            _ => (0, 10000),
        }
    }

    /// Readjust description parser x_bounds if not already done and
    /// headers are all set
    fn adjust_description_x_bounds(&mut self) {
        if self.description_x_bounds_adjusted {
            return;
        }
        if !self.all_headers_set() {
            return;
        }
        // Get effective x_bounds from other parsers
        for field in &self.all_fields {
            if field == "description" {
                continue;
            }
            let (x_lower, x_upper) = self.get_parser_x_bounds(field);
            self.description_parser.adjust_bounds(x_lower, x_upper);
            self.description_parser_stray
                .adjust_bounds(x_lower, x_upper);
        }
    }

    /// Mirror the description parser's header-derived x1/x2 bounds onto the
    /// stray parser, since the stray parser never sees header items directly
    fn sync_description_stray_bounds(&mut self) {
        if self.description_parser.is_header_set() {
            let (x1_range, x2_range) = self.description_parser.bounds();
            self.description_parser_stray.set_bounds(x1_range, x2_range);
        }
    }

    /// Merge any buffered stray description text into the last completed
    /// transaction. Called when a new transaction starts or the transaction
    /// table ends, since the stray text most likely belongs to the previous
    /// (vertically centered) transaction.
    fn merge_stray_into_last(&mut self, data: &mut AccountData) {
        let Some(text) = self.description_parser_stray.take_stray() else {
            return;
        };
        let Some(last) = data.proto_transactions.last_mut() else {
            return;
        };
        if !last.description.is_empty() {
            last.description.push(' ');
        }
        last.description.push_str(&text);
        // Stray text bypasses the normal push-time cleanup, so re-apply it here
        last.clean_description(&self.description_exclude_patterns);
    }

    /// Check if the current items indicate a new line
    fn is_new_line(&self, items: &[TextItem]) -> bool {
        if items.is_empty() {
            return false;
        }
        items[0].y1_bin != self.current_line_y1_bin
    }

    /// Prime all specified parsers
    fn prime_parsers(&mut self, fields: Vec<String>) {
        for field in fields {
            match field.as_str() {
                "date" => self.date_parser.prime(),
                "description" => self.description_parser.prime(),
                "amount" => self.amount_parser.prime(),
                "balance" => self.balance_parser.prime(),
                _ => {}
            }
        }
    }

    /// Reset all parsers (unprime and reset)
    fn reset_all_parsers(&mut self) {
        self.date_parser.reset();
        self.description_parser.reset();
        self.amount_parser.reset();
        self.balance_parser.reset();
    }

    /// Prime new line fields, un-prime the rest.
    /// Fields after description are also primed if a new line field.
    fn prime_new_line_fields(&mut self) {
        self.reset_all_parsers();
        self.prime_parsers(self.new_line_fields.clone());

        if self.new_line_fields.contains(&"description".to_string())
            && let Some(next_fields) = self.next_fields.get("description")
        {
            self.prime_parsers(next_fields.clone());
        }
    }

    /// Append current transaction to statement data if all compulsory fields are set
    fn append_current_transaction(&mut self, data: &mut AccountData) {
        if !self
            .current_transaction
            .has_required_fields_set(&self.compulsory_fields)
        {
            return;
        }
        self.current_transaction
            .clean_description(&self.description_exclude_patterns);
        data.proto_transactions
            .push(self.current_transaction.clone());
    }

    /// Handle post-parse actions after a field is successfully parsed
    fn post_parse_append(&mut self, field: String, data: &mut AccountData) {
        if !self.end_line_fields.contains(&field) {
            return;
        }
        self.append_current_transaction(data);
        self.merge_stray_into_last(data);
        self.current_transaction = ProtoTransaction::new();
        // Needed if previous field was description
        self.description_parser.reset();
    }

    /// Handle post-parse priming after a field is successfully parsed
    fn post_parse_prime(&mut self, field: String) {
        self.reset_all_parsers();
        let next_fields_vec = self.next_fields.get(&field).cloned().unwrap_or_default();
        self.prime_parsers(next_fields_vec.clone());
        // Prime field after description if it is a next field
        if next_fields_vec.contains(&"description".to_string())
            && let Some(desc_next_fields_vec) = self.next_fields.get("description").cloned()
        {
            self.prime_parsers(desc_next_fields_vec);
        }
    }

    /// Handle new line parsing for specified fields
    fn handle_new_line(&mut self, items: &[TextItem], data: &mut AccountData) -> usize {
        if !self.description_parser.primed {
            self.prime_new_line_fields();
        }

        // Handle Date field
        if self.new_line_fields.contains(&"date".to_string()) {
            self.date_parser.reset();
            self.date_parser_newline.reset();
            self.date_parser_newline.prime();
            let mut next_transaction: ProtoTransaction = ProtoTransaction::new();
            let date_consumed = self
                .date_parser_newline
                .parse_items(items, &mut next_transaction);
            if date_consumed > 0 {
                self.append_current_transaction(data);
                self.merge_stray_into_last(data);
                self.current_transaction = next_transaction;
                self.description_parser.reset();
                self.post_parse_prime("date".to_string());
                return date_consumed;
            }
        }

        // Handle Amount field
        if self.new_line_fields.contains(&"amount".to_string()) {
            self.amount_parser.reset();
            self.amount_parser_newline.reset();
            self.amount_parser_newline.prime();
            let mut next_transaction: ProtoTransaction = ProtoTransaction::new();
            let amount_consumed = self
                .amount_parser_newline
                .parse_items(items, &mut next_transaction);
            if amount_consumed > 0 {
                self.append_current_transaction(data);
                self.merge_stray_into_last(data);
                self.current_transaction = next_transaction;
                self.description_parser.reset();
                self.post_parse_prime("amount".to_string());
                return amount_consumed;
            }
        }

        // Handle Balance field
        if self.new_line_fields.contains(&"balance".to_string()) {
            self.balance_parser.reset();
            self.balance_parser_newline.reset();
            self.balance_parser_newline.prime();
            let mut next_transaction: ProtoTransaction = ProtoTransaction::new();
            let balance_consumed = self
                .balance_parser_newline
                .parse_items(items, &mut next_transaction);
            if balance_consumed > 0 {
                self.append_current_transaction(data);
                self.merge_stray_into_last(data);
                self.current_transaction = next_transaction;
                self.description_parser.reset();
                self.post_parse_prime("balance".to_string());
                return balance_consumed;
            }
        }
        0
    }
}

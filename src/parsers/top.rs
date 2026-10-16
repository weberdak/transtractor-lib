use crate::parsers::statement::{
    AccountNumberParser, ClosingBalanceParser, OpeningBalanceParser, StartDateParser,
    TransactionParser,
};
use crate::structs::AccountData;
use crate::structs::TextItem;
use crate::structs::{Benchmark, StatementConfig};

/// Top-level function that converts a list of TextItems into structured AccountData
pub fn parse_text_items(config: &StatementConfig, text_items: &[TextItem]) -> AccountData {
    let mut benchmark = Benchmark::new();
    benchmark.total.start();
    let data = parse_text_items_with_benchmark(config, text_items, &mut benchmark);
    benchmark.total.pause();
    let mut data = data;
    data.benchmark = benchmark;
    data
}

pub fn parse_text_items_with_benchmark(
    config: &StatementConfig,
    text_items: &[TextItem],
    benchmark: &mut Benchmark,
) -> AccountData {
    let mut statement_data = AccountData::new();

    // Initialize parsers
    let mut account_number_parser = AccountNumberParser::new(config);
    let mut opening_balance_parser = OpeningBalanceParser::new(config);
    let mut closing_balance_parser = ClosingBalanceParser::new(config);
    let mut start_date_parser = StartDateParser::new(config);
    let mut transaction_parser = TransactionParser::new(config);

    // Other settings based on parsers
    // Compute max lookahead across all parsers generically to keep this scalable
    let lookaheads = [
        account_number_parser.get_max_lookahead(),
        opening_balance_parser.get_max_lookahead(),
        closing_balance_parser.get_max_lookahead(),
        start_date_parser.get_max_lookahead(),
        transaction_parser.get_max_lookahead(),
    ];
    let max_lookahead = *lookaheads.iter().max().unwrap_or(&0);

    // Iterate through text items, attempting to match account_terms
    let len = text_items.len();
    if len == 0 {
        statement_data.benchmark = benchmark.clone();
        return statement_data;
    }
    let mut i: usize = 0;
    while i < len {
        let buffer_size = max_lookahead.min(len - i);
        let buffer = &text_items[i..i + buffer_size];
        let mut consumed = 0usize;
        // Try parsers in a stable order: account number -> start date -> opening balance -> closing balance
        if consumed == 0 {
            benchmark.parsers_account_number_parser_parse.start();
            consumed = account_number_parser.parse_items_timed(
                buffer,
                &mut statement_data,
                &mut benchmark.parsers_account_number_parser_prime,
            );
            benchmark.parsers_account_number_parser_parse.pause();
        }
        if consumed == 0 {
            benchmark.parsers_start_date_parser_parse.start();
            consumed = start_date_parser.parse_items_timed(
                buffer,
                &mut statement_data,
                &mut benchmark.parsers_start_date_parser_prime,
            );
            benchmark.parsers_start_date_parser_parse.pause();
        }
        if consumed == 0 && (!config.opening_balance_inside_table || transaction_parser.is_primed())
        {
            benchmark.parsers_opening_balance_parser_parse.start();
            consumed = opening_balance_parser.parse_items_timed(
                buffer,
                &mut statement_data,
                &mut benchmark.parsers_opening_balance_parser_prime,
            );
            benchmark.parsers_opening_balance_parser_parse.pause();
        }
        if consumed == 0
            && (!config.closing_balance_inside_table
                || transaction_parser.is_primed()
                || closing_balance_parser.is_primed())
        {
            benchmark.parsers_closing_balance_parser_parse.start();
            consumed = closing_balance_parser.parse_items_timed(
                buffer,
                &mut statement_data,
                &mut benchmark.parsers_closing_balance_parser_prime,
            );
            benchmark.parsers_closing_balance_parser_parse.pause();
        }
        if consumed == 0 {
            benchmark.parsers_transaction_parser_parse.start();
            consumed = transaction_parser.parse_items_timed(
                buffer,
                &mut statement_data,
                &mut benchmark.parsers_transaction_parser_start_prime,
                &mut benchmark.parsers_transaction_parser_stop_prime,
            );
            benchmark.parsers_transaction_parser_parse.pause();
        }
        // A parser can signal an unrecoverable config error via usize::MAX, logged
        // into statement_data.errors; stop parsing early in that case.
        if consumed == usize::MAX {
            break;
        }
        if consumed > 0 {
            i += consumed;
            continue;
        }

        // No parser matched, move to next item
        i += 1;
    }

    // Wipe transaction balances if ignored
    if config.transaction_balance_ignore {
        for transaction in &mut statement_data.proto_transactions {
            transaction.balance = None;
        }
    }

    // Closing balance is taken from the running balance of the last transaction
    if config.closing_balance_set_from_last_transaction {
        let balance = statement_data
            .proto_transactions
            .last()
            .and_then(|transaction| transaction.balance);
        statement_data.closing_balance = balance;
    }

    statement_data.benchmark = benchmark.clone();
    statement_data
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(text: &str, y1: i32) -> TextItem {
        TextItem {
            text: text.to_string(),
            x1: 100,
            y1,
            x2: 110,
            y2: y1 + 10,
            page: 1,
            y1_bin: y1,
        }
    }

    fn base_config() -> StatementConfig {
        StatementConfig {
            transaction_terms: vec!["Transactions".to_string()],
            transaction_terms_stop: vec!["End table".to_string()],
            opening_balance_terms: vec!["Opening balance".to_string()],
            opening_balance_formats: vec!["format1".to_string()],
            opening_balance_alignment: "x1".to_string(),
            closing_balance_terms: vec!["Closing balance".to_string()],
            closing_balance_formats: vec!["format1".to_string()],
            closing_balance_alignment: "x1".to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn opening_balance_inside_table_ignores_pre_table_value() {
        let mut config = base_config();
        config.opening_balance_inside_table = true;
        let items = vec![
            item("Opening", 10),
            item("balance", 10),
            item("100.00", 20),
            item("Transactions", 30),
            item("Opening", 40),
            item("balance", 40),
            item("200.00", 50),
        ];

        let data = parse_text_items(&config, &items);

        assert_eq!(data.opening_balance(), Some(200.0));
    }

    #[test]
    fn closing_balance_inside_table_ignores_pre_table_value() {
        let mut config = base_config();
        config.closing_balance_inside_table = true;
        let items = vec![
            item("Closing", 10),
            item("balance", 10),
            item("100.00", 20),
            item("Transactions", 30),
            item("Closing", 40),
            item("balance", 40),
            item("200.00", 50),
        ];

        let data = parse_text_items(&config, &items);

        assert_eq!(data.closing_balance(), Some(200.0));
    }
}

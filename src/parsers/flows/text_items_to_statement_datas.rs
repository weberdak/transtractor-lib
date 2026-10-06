use crate::checkers::check_statement_data;
use crate::fixers::fix_statement_data;
use crate::parsers::top::parse_text_items_with_benchmark;
use crate::structs::StatementConfig;
use crate::structs::TextItem;
use crate::structs::text_items::tokenise_items;
use crate::structs::{AccountData, Benchmark};

/// Extract AccountData objects from text items using provided statement configurations.
pub fn text_items_to_statement_datas(
    items: &Vec<TextItem>,
    configs: &Vec<StatementConfig>,
    exit_when_succeed: bool,
) -> Result<Vec<AccountData>, String> {
    let mut benchmark = Benchmark::new();
    text_items_to_statement_datas_with_benchmark(items, configs, exit_when_succeed, &mut benchmark)
}

pub fn text_items_to_statement_datas_with_benchmark(
    items: &Vec<TextItem>,
    configs: &Vec<StatementConfig>,
    exit_when_succeed: bool,
    benchmark: &mut Benchmark,
) -> Result<Vec<AccountData>, String> {
    let mut results = Vec::new();
    if configs.is_empty() {
        benchmark.total.pause();
        return Ok(results);
    }
    for cfg in configs {
        benchmark.total.start();
        benchmark.tokeniser.start();
        let tokenised_items = tokenise_items(items);
        benchmark.tokeniser.pause();
        benchmark.parsers.start();
        let mut data = parse_text_items_with_benchmark(cfg, &tokenised_items, benchmark);
        benchmark.parsers.pause();
        data.set_key(cfg.key.clone());

        // Apply fixers to clean up the data
        benchmark.fixers.start();
        fix_statement_data(&mut data);
        benchmark.fixers.pause();
        benchmark.checkers.start();
        check_statement_data(&mut data);
        benchmark.checkers.pause();
        benchmark.total.pause();
        data.benchmark = benchmark.clone();

        // Return early if an error-free AccountData is found
        if exit_when_succeed && data.errors.is_empty() {
            results.push(data);
            break;
        }

        results.push(data);
    }

    Ok(results)
}

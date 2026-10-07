use crate::configs::db::ConfigDB;
use crate::parsers::flows::text_items_to_statement_datas::text_items_to_statement_datas_with_benchmark;
use crate::structs::{Benchmark, StatementData, TextItem};

/// Top-level workflow to parse extracted text items into structured statement data
pub fn text_items_to_statement_data(
    config_db: &ConfigDB,
    items: &Vec<TextItem>,
) -> Result<StatementData, String> {
    let mut benchmark = Benchmark::new();
    text_items_to_statement_data_with_benchmark(config_db, items, &mut benchmark)
}

pub fn text_items_to_statement_data_with_benchmark(
    config_db: &ConfigDB,
    items: &Vec<TextItem>,
    benchmark: &mut Benchmark,
) -> Result<StatementData, String> {
    benchmark.start_total();
    let configs = config_db.identify_with_benchmark(items, benchmark);

    // User error: trying to parse unsupported bank statement format
    if configs.is_empty() {
        benchmark.total.pause();
        return Err("Bank statement format cannot be identified.".to_string());
    }

    // Pool every error-free AccountData; duplicates are consolidated on import
    let account_data_results =
        text_items_to_statement_datas_with_benchmark(items, &configs, false, benchmark)?;
    let mut statement_data = StatementData::new();
    let mut parsed = false;
    for data in &account_data_results {
        if data.errors.is_empty() {
            if let Err(e) = statement_data.import(data) {
                benchmark.total.pause();
                return Err(e);
            }
            parsed = true;
        }
    }
    if parsed {
        statement_data.benchmark = benchmark.clone();
        return Ok(statement_data);
    }
    benchmark.total.pause();

    // Software bug: If statement is recognised, it should be parsed successfully
    let keys: Vec<String> = configs.iter().map(|cfg| cfg.key.clone()).collect();
    Err(format!(
        "Bank statement recognised but cannot be parsed. Debug configurations: {:?}",
        keys
    ))
}

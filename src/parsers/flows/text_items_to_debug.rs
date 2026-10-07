use crate::configs::db::ConfigDB;
use crate::parsers::flows::text_items_to_account_datas::text_items_to_account_datas_with_benchmark;
use crate::structs::{Benchmark, TextItem};

/// Parse non-tokenised text items into debug information string,
/// using provided statement configurations.
pub fn text_items_to_debug(config_db: &ConfigDB, items: &Vec<TextItem>) -> Result<String, String> {
    let mut benchmark = Benchmark::new();
    text_items_to_debug_with_benchmark(config_db, items, &mut benchmark)
}

pub fn text_items_to_debug_with_benchmark(
    config_db: &ConfigDB,
    items: &Vec<TextItem>,
    benchmark: &mut Benchmark,
) -> Result<String, String> {
    benchmark.start_total();
    let configs = config_db.identify_with_benchmark(items, benchmark);

    // User error: trying to parse unsupported bank statement format
    if configs.is_empty() {
        benchmark.total.pause();
        return Err("Bank statement format cannot be identified.".to_string());
    }

    // Write debug information to the output file
    let mut output = String::new();
    output.push_str("Debug output\n");

    match text_items_to_account_datas_with_benchmark(items, &configs, false, benchmark) {
        Ok(statement_data_results) => {
            output.push_str(&format!(
                "Found {} StatementData result(s)\n\n",
                statement_data_results.len()
            ));

            for (i, data) in statement_data_results.iter().enumerate() {
                output.push_str(&format!("=== StatementData Result {} ===\n", i + 1));
                output.push_str(&data.to_string());
                output.push('\n');
            }
        }
        Err(e) => {
            return Err(format!("Unexpected error: {}", e));
        }
    }
    Ok(output)
}

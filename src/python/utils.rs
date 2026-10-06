use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList};

/// Convert a Rust StatementData to a Python StatementData object
pub fn rust_statement_data_to_py_statement_data(
    rust_statement_data: &crate::structs::StatementData,
) -> PyResult<Py<PyAny>> {
    Python::attach(|py| {
        // Import the Python StatementData and Transaction classes
        let statement_data_module = py.import("transtractor.structs.statement_data")?;
        let statement_data_class = statement_data_module.getattr("StatementData")?;

        let transaction_module = py.import("transtractor.structs.transaction")?;
        let transaction_class = transaction_module.getattr("Transaction")?;
        let benchmark_module = py.import("transtractor.structs.benchmark")?;
        let benchmark_class = benchmark_module.getattr("Benchmark")?;

        let py_transactions = PyList::empty(py);
        for tx in &rust_statement_data.transactions {
            let py_transaction = transaction_class.call1((
                tx.date,
                tx.index,
                tx.description.clone(),
                tx.amount,
                tx.balance,
                tx.account_number.clone(),
            ))?;
            py_transactions.append(py_transaction)?;
        }

        // Create Python StatementData object.
        // The parser wrapper can overwrite the blank filename later if needed.
        let kwargs = PyDict::new(py);
        kwargs.set_item("filename", "")?;
        kwargs.set_item("transactions", py_transactions)?;
        let benchmark = rust_statement_data.benchmark.as_micros();
        let benchmark_kwargs = PyDict::new(py);
        benchmark_kwargs.set_item("total", benchmark.total)?;
        benchmark_kwargs.set_item("pdf_extractor", benchmark.pdf_extractor)?;
        benchmark_kwargs.set_item("tokeniser", benchmark.tokeniser)?;
        benchmark_kwargs.set_item("typer", benchmark.typer)?;
        benchmark_kwargs.set_item("parsers", benchmark.parsers)?;
        benchmark_kwargs.set_item(
            "parsers_account_number_parser_prime",
            benchmark.parsers_account_number_parser_prime,
        )?;
        benchmark_kwargs.set_item(
            "parsers_account_number_parser_parse",
            benchmark.parsers_account_number_parser_parse,
        )?;
        benchmark_kwargs.set_item(
            "parsers_start_date_parser_prime",
            benchmark.parsers_start_date_parser_prime,
        )?;
        benchmark_kwargs.set_item(
            "parsers_start_date_parser_parse",
            benchmark.parsers_start_date_parser_parse,
        )?;
        benchmark_kwargs.set_item(
            "parsers_opening_balance_parser_prime",
            benchmark.parsers_opening_balance_parser_prime,
        )?;
        benchmark_kwargs.set_item(
            "parsers_opening_balance_parser_parse",
            benchmark.parsers_opening_balance_parser_parse,
        )?;
        benchmark_kwargs.set_item(
            "parsers_closing_balance_parser_prime",
            benchmark.parsers_closing_balance_parser_prime,
        )?;
        benchmark_kwargs.set_item(
            "parsers_closing_balance_parser_parse",
            benchmark.parsers_closing_balance_parser_parse,
        )?;
        benchmark_kwargs.set_item(
            "parsers_transaction_parser_start_prime",
            benchmark.parsers_transaction_parser_start_prime,
        )?;
        benchmark_kwargs.set_item(
            "parsers_transaction_parser_parse",
            benchmark.parsers_transaction_parser_parse,
        )?;
        benchmark_kwargs.set_item(
            "parsers_transaction_parser_stop_prime",
            benchmark.parsers_transaction_parser_stop_prime,
        )?;
        benchmark_kwargs.set_item("fixers", benchmark.fixers)?;
        benchmark_kwargs.set_item("checkers", benchmark.checkers)?;
        kwargs.set_item(
            "benchmark",
            benchmark_class.call((), Some(&benchmark_kwargs))?,
        )?;
        let py_statement_data = statement_data_class.call((), Some(&kwargs))?;

        Ok(py_statement_data.into())
    })
}

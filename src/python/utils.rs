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
        let py_statement_data = statement_data_class.call((), Some(&kwargs))?;

        Ok(py_statement_data.into())
    })
}

# The Transtractor

![PyPI version](https://img.shields.io/pypi/v/transtractor)
![Development Status](https://img.shields.io/pypi/status/transtractor)
![Tests](https://github.com/weberdak/transtractor-lib/actions/workflows/ci.yml/badge.svg)
![Read the Docs](https://readthedocs.org/projects/transtractor-lib/badge/?version=latest)
![codecov](https://codecov.io/gh/transtractor/transtractor-lib/branch/main/graph/badge.svg)
![License](https://img.shields.io/github/license/transtractor/transtractor-lib)


## The Universal Bank Statement PDF Extractor

The Transtractor is a rules-based PDF bank statement parser for extracting transaction data from [supported statements](https://transtractor-lib.readthedocs.io/en/latest/supported_statements.html). It combines a fast Rust parsing engine with Python and WebAssembly bindings, and a [web-based GUI](https://www.transtractor.net/).

### Key features

* Parse bank statement PDFs with deterministic, configuration-driven rules
* Normalise extracted data into consistent dates, descriptions, amounts and balances
* Extract transactions from multiple accounts within a single statement
* Infer missing transaction dates and running balances
* Normalise transaction signs so debits are negative and credits are positive
* Validate transaction totals against opening and closing balances
* Process statements at a throughput of 10 to 20 statements per second on average

Each supported statement format is implemented as a lightweight configuration module. This design keeps the parser portable and extensible while making its output predictable and suitable for downstream processing, reporting, and financial analysis.

## Installation

### Install from PyPI

The Transtractor is available on PyPI and can be installed with pip:

```shell
pip install transtractor
```

### Compile from Source

1. **Install Rust**: Download and install Rust from [rustup.rs](https://rustup.rs/)

2. **Install uv**: Follow instructions from [Astral](https://docs.astral.sh/uv/getting-started/installation/)

3. **Sync Python environment and compile**: Clone the repository and build
   ```shell
   git clone https://github.com/transtractor/transtractor.git
   cd transtractor-lib
   uv sync --locked --group dev
   ```

4. **Test the package**: Run Rust and Python unit tests
   ```shell
   cargo test
   uv run pytest
   ```

### Basic Usage

The detailed [documentation](https://transtractor-lib.readthedocs.io/en/latest/) is maintained on Read the Docs, but you can get started with these steps.

1. **Import and initialise the parser**
   ```python
   from transtractor import Parser

   parser = Parser()
   ```

2. **Parse a PDF**: Extract transaction data into a `StatementData` object
   ```python
   statement_data = parser.parse('statement.pdf')
   ```

   The Rust engine performs all extraction and validation. `ParseError` is raised if validation fails or the statement is not recognised.

   `Transaction` objects listed in `statement_data.transactions` have the following attributes:
   * **date**: Transaction date in UTC (*datetime.date*)
   * **date_index**: Index of the transaction on that date (*int*)
   * **description**: Transaction description (*str*)
   * **amount**: Money out (debit) is negative, money in (credit) is positive (*float*)
   * **balance**: Positive if you have money, negative if you owe it (*float*)
   * **account_number**: Unique identifier for the account, may be incomplete (*str*)

   The Rust engine infers missing dates and running balances.

3. **Write to CSV**: All CSV files are written in a fixed format
   ```python
   statement_data.to_csv('statement.csv')
   ```

   Example output:
   ```csv
   date,description,amount,balance,account_number
   2025-01-01,Transaction 1,50000.0,100000.0,1234 5678 9123 4567
   2025-01-01,Transaction 2,-1000.0,99000.0,1234 5678 9123 4567
   2025-01-01,Transaction 3,-10000.0,89000.0,1234 5678 9123 4567
   2025-01-01,Transaction 4,1350.0,90350.0,1234 5678 9123 4567
   2025-01-03,Transaction 5,-530.99,89819.01,1234 5678 9123 4567
   2025-01-03,Transaction 6,1532.55,91351.56,1234 5678 9123 4567
   2025-01-04,Transaction 7,-568.01,90783.55,1234 5678 9123 4567
   2025-01-04,Transaction 8,-23.56,90759.99,1234 5678 9123 4567
   2025-01-04,Transaction 9,-2000.0,88759.99,1234 5678 9123 4567
   ...
   ```

   The `date_index` field is not useful in this context.


3. **Convert PDF to DataFrame**: For analysis with pandas or Polars
   ```python
   import pandas as pd

   dict_data = statement_data.to_pandas_dict()
   df = pd.DataFrame(dict_data)
   ```

### Batch Processing

The Python API does not provide a high-level batch-processing interface. However, `Transaction` objects support set operations, which can be used to combine transactions and remove duplicates. Two transactions compare as equal when their *date*, *date_index*, *amount*, and *account_number* attributes match; `__eq__` and `__hash__` implement this behaviour.

The recipe below parses PDF files in a directory and its subdirectories, aggregates transactions into a set, and writes to a CSV file:

```python
import csv
from pathlib import Path

from transtractor import Parser

# Paths to the PDF directory and output CSV file
pdf_directory = Path("path/to/your/pdf/directory")
output_csv = Path("output.csv")

parser = Parser()
transactions = set()

# Find and extract PDFs in directory and subdirectories
for pdf_path in sorted(pdf_directory.rglob("*.pdf")):
   print(f"Parsing PDF: {pdf_path}")
   statement_data = parser.parse(str(pdf_path))
   transactions.update(statement_data.transactions)

# Arrange newest to oldest
ordered_transactions = sorted(
   transactions,
   key=lambda transaction: (
      transaction.date,
      transaction.date_index,
      transaction.account_number,
   ),
   reverse=True,
)

# Write the ordered transactions to the output CSV file
with output_csv.open("w", newline="", encoding="utf-8") as csv_file:
   writer = csv.writer(csv_file)
   writer.writerow(("date", "description", "amount", "balance", "account_number"))
   for transaction in ordered_transactions:
      writer.writerow(
            (
               transaction.date,
               transaction.description,
               transaction.amount,
               transaction.balance,
               transaction.account_number,
            )
      )
```

## Supported Statements

See the documentation for a current list of [supported statements](https://transtractor-lib.readthedocs.io/en/latest/supported_statements.html). If your statement is not yet supported, you may be able to create your own parsing configuration files by following these [instructions](https://transtractor-lib.readthedocs.io/en/latest/configuration.html). Load them with:

```python
from transtractor import Parser

parser = Parser()
parser.load('my_config.json')
statement_data = parser.parse('statement.pdf')
```

The Transtractor is still in early development and may not support the parameters or formatting patterns needed to parse your statement. It may be quicker and easier to send a [redacted *layout file*](https://www.transtractor.net/add-support-for-your-statements). You can generate one in Python with:

```python
parser.layout("statement.pdf", "statement.layout.txt")
```

## Performance

The Transtractor typically parses between 10 and 20 statements per second on average. Parsing time scales linearly with the number of transactions in each statement. Actual performance varies by statement type and the density of extractable information. Statements with a higher proportion of non-extractable content create more noise for the parser to filter through.

![Parsing performance vs. number of transactions](md/performance.svg)

## WASM Implementation

WASM bindings are also provided for in-browser parsing of PDF bank statements. See [this guide](md/wasm.md) for an introduction to compiling and using them.

You may also want to check out [www.transtractor.net](https://www.transtractor.net) to see these bindings in action.

## Developers

The following pages provide further information about how this package is built and developed:

* [Architecture Guide](md/architecture.md): Overview of key application components and design principles.
* [Developer Guide](md/develop.md): Reference page for core development and maintenance.
* [Contributor Guide](CONTRIBUTING.md): Extending the package to parse additional bank statements.
* [WASM Guide](md/wasm.md): Build and usage notes for the TypeScript/WASM package.

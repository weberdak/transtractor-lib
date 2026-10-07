# Architecture Guide
The Transtractor is implemented in **Python** and **Rust**. Rust is used not primarily for performance, but to ensure the core processing engine can be ported across multiple languages and runtimes. Python bindings make it easy to integrate Transtractor into backend systems for data extraction and analysis. WASM bindings also support static web applications such as [Transtractor.net](https://www.transtractor.net/), for secure, in‑browser parsing of bank statements.

## Parsing Pipeline
Transtractor first extracts PDF text into **tokenised word units**, which are then passed to the **processing engine**
using key terms, token order, and token coordinates to reconstruct transaction records and metadata such as account numbers and opening/closing balances. The Rust engine performs extensive validation before returning structured results to the high‑level language for further analysis.

### Step 1: Extract PDF Content
PDF text extraction is performed using the *pdfsink-rs* Rust package, which reliably handles many fonts, whitespace, and bounding‑box coordinates.

All Python‑side parsing is encapsulated in the *Parser* class. When instantiated, it automatically loads the **standard configuration database** from the regionally-organised Rust modules under *src/configs/registry*. A single `Parser` instance can process multiple statements and its internal database can be extended or modified by imported user-supplied configurations in JSON format.

The first stage implemented by `pdf_to_text_items` extracts all PDF text into **ordered word blocks**, which are then split by whitespace into **single word tokens** before being fed into the **processing engine**.

The processing engine processes statements in two parts:

1. **Statement classification** determines the statement type using a keyword-based algorithm implemented by the `StatementTyper`.
2. **Transaction extraction** processes tokens into structured transaction data. 

### Step 2: Process tokens into Structured Transaction Data
Extracts transaction data according to one or more candidate statement types returned by the `StatementTyper`.

The top‑level entry point, `text_items_to_statement_data`, identifies the relevant configurations and passes them to `text_items_to_account_datas`. That function tokenises the items and, for each configuration, parses, fixes and checks them into an `AccountData` object holding the results for a single account. Fixers repair recoverable problems (for example implicit dates and balances), and checkers validate field completeness and balance continuity, recording any problems in the object's `errors`.

This iterative behaviour is essential because statement formats evolve over time, and multiple versions may share identical keyword signatures. Every configuration is attempted, so the correct format is found even when classification alone is ambiguous. A single statement may also contain several accounts, each parsed successfully by a different configuration.

Every error‑free `AccountData` is then imported into a single `StatementData` object, a flat pool of `Transaction` records, each tagged with its account number and per‑date index. A transaction is unique by its date, index, amount, balance and account number (descriptions are ignored), so duplicates produced by multiple parsing attempts are consolidated. Import fails if an `AccountData` is incomplete. If no configuration yields error‑free data, an error is returned.

The pooled result is returned to Python through `rust_statement_data_to_py_statement_data`, the **Rust‑to‑Python output interface** (the WASM bindings provide an equivalent). All subsequent processing and data handling occur on the Python side using the `StatementData` transfer class.

The Transtractor’s core parsing model separates **reusable logic** from **format‑specific rules**. All general parsing behaviour is implemented directly in the Rust engine, while statement‑specific formatting details are defined in lightweight configuration modules or JSON files. This avoids building a bespoke parser for every individual statement format. Even though statements can look very different, they share structural patterns that the Transtractor can exploit and eventually evolve into a genuinely universal parser.

### Step 3: Using the Data
The Python‑side `StatementData` transfer class provides methods for exporting results either as CSV files or as Pandas‑compatible dictionaries, making the data easy to load into spreadsheet tools or integrate into broader analytical workflows. This is where the scope of this repository ends as the downstream use of the extracted data is intentionally open‑ended and entirely up to the user.

For me, this parsing library is a foundational component of the [Transtractor.net](https://www.transtractor.net/) personal budgeting app I build in my spare time. Other users may choose to incorporate it into their own personal finance tools or into business workflows that require routine, reliable extraction of financial data from bank statements.

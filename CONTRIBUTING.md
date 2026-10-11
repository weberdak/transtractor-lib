# Contributing to the Transtractor

Thank you for your interest in contributing to the Transtractor. Contributions that add support for additional statement formats are especially welcome, since genuine bank statements are difficult to obtain for testing. Small bug fixes are also welcome; however, changes to the core parsing logic or architecture are generally reserved for maintainers.

This guide covers the usual contributor workflow. For deeper implementation and maintenance details, see the [development guide](md/develop.md). For an overview of the codebase, see the [architecture guide](md/architecture.md).

## Set up the project

You will need:

* [Rust](https://rust-lang.org/tools/install/)
* [Node.js](https://nodejs.org/) for the WebAssembly package
* [uv](https://docs.astral.sh/uv/getting-started/)
* [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) for WebAssembly builds

The repository also includes a VS Code Dev Container with the project tooling preconfigured. With Docker and the Dev Containers extension installed, use **Dev Containers: Reopen in Container** from the Command Palette.

For a local installation, clone the repository and sync all dependencies:

```shell
git clone https://github.com/weberdak/transtractor-lib.git
cd transtractor-lib
uv sync --locked --all-groups
cd wasm
npm ci
```

## Choose the right guide

* To add support for a bank statement format, follow the [statement support workflow](#adding-statement-support).
* To change the Rust parser, Python API, or shared data structures, read the [architecture guide](md/architecture.md) first and add focused tests alongside the change.
* To change the browser or TypeScript interface, see the [WASM guide](md/wasm.md).
* To change user-facing documentation, update the relevant files under `docs/` and build the documentation locally.

## Adding statement support

New statement support generally consists of a configuration, a redacted spec fixture, and any required date or amount formats. The `Parser.test()` method of the Python API searches the supplied directory and its subdirectories for PDFs and tests whether they can be successfully parsed.

1. Create and test a JSON configuration using the documented [configuration reference](https://transtractor-lib.readthedocs.io/en/latest/configuration.html):

	```python
	from transtractor import Parser

	parser = Parser()
	parser.load("my_config.json")
	parser.test("path/to/statement/pdfs")
	```

2. Convert the configuration into a Rust module under `src/configs/registry/<region>/`. The module filename must match the configuration `key`; register it in the region's `mod.rs`. Register new regions in the `regions` vector in `src/configs/registry/mod.rs`. If the format requires new date or amount formats, add and register them in `src/formats`.

	Rebuild the extension after adding the configuration:

	```shell
	uv run maturin develop --release
	uv sync --locked --group dev
	```

3. To contribute to the project, at least one representative *spec* file is required. A spec file is a special testing fixture that stores the ordered text and coordinates extracted from a PDF together with the expected parsing result. These files are easy to redact and are a safer alternative to real PDF statements. To create these, first create a *layout* file from a representative PDF statement:

	```python
	parser.layout("path/to/representative/statement.pdf", "statement_layout.txt")
	```

	Redact your personal information following the instructions on the [website](https://www.transtractor.net/add-support-for-your-statements). Note that you need to enure that the dummy amounts and balances are balanced before creating the spec file. The easiest way to validate this is by the `debug_layout` method:

	```python
	parser.debug_layout("statement_layout.txt", "statement_debug.txt")
	```

	If no errors are encountered, generate the spec file:

	```python
	parser.spec_layout(
		 "statement_layout.txt",
		 "{bank-code}__{bank-product}__{spec-version}.json",
	)
	```

	Store the resulting JSON under `tests/fixtures/spec/<region>/`. The filename must have exactly three lowercase components separated by double underscores: `<bank-code>__<spec-name>__<integer-version>.json`. The region directory and bank-code component must match the start of at least one configuration `key` (for example, `au/cba__savings__1.json` requires a key starting with `au__cba__`).

4. Run the spec tests with `cargo test`. They check fixture naming, placement, and exact parsing behaviour.

5. Email the spec file separately to develop@transtractor.net once a pull request is opened. This allows maintainers to verify accuracy ensure the parsing of your statements are not corrupted in future releases. These files are not committed to the repository in case personal information has not been properly redacted.

## Validate your changes

Run the checks relevant to the code you changed. Before opening a pull request, the core checks are:

```shell
cargo test
uv run pytest
uv run ruff check python scripts tests
uv run ruff format --check python scripts tests
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
```

For WASM changes, also run:

```shell
cd wasm
npm run lint
npm run test
npm run build
```

For documentation changes, build the Sphinx site:

```shell
cd docs
uv run make html
```

See [md/develop.md](md/develop.md) for coverage, type checking, dependency audits, CI details, and release procedures.

## Commit messages

Use a short, lowercase commit type followed by a colon and an imperative description:

```text
<type>: <short description>
```

Use the type that best describes the change:

* `feat` — add user-facing functionality or support for a new statement format.
* `fix` — correct a bug or regression.
* `docs` — update documentation or other explanatory content.
* `chore` — perform maintenance, such as updating dependencies or the project version.
* `breaking` — make an incompatible API or behaviour change.

Keep each commit focused. Recent examples include `feat: add support for US Capital One 360 statements`, `docs: document new transaction_balance_ignore param`, and `chore: bump to v0.14.0`.

## Pull requests

Before submitting a pull request:

* Keep the change focused and explain the user-visible or maintenance benefit.
* Add or update tests and fixtures for behavioural changes.
* Check that generated or sensitive files are not included.
* Confirm that formatting, linting, tests, and relevant builds pass locally.
* Update documentation when public behaviour or contributor workflow changes.

Open the pull request from your fork against `main`. GitHub Actions will run the project test, build, lint, type-checking, and audit workflows. Please address failures before requesting review.

Use the following template in the pull-request description:

```markdown
## Summary
One or a few high-level sentences summarising the release. This should be a concise overview of the changes, improvements, or fixes included in this release.

## Changes
### Breaking changes
* One-sentence descriptions without full stops

### Bug fixes
* One-sentence descriptions without full stops

### New features
* One-sentence descriptions without full stops

### Performance enhancements
* One-sentence descriptions without full stops

### Security updates
* One-sentence descriptions without full stops

### Refactoring and code quality improvements
* One-sentence descriptions without full stops

### Documentation updates
* One-sentence descriptions without full stops

### CI/CD improvements
* One-sentence descriptions without full stops

### Issue resolutions
* One-sentence descriptions without full stops

### Miscellaneous
* One-sentence descriptions without full stops
```

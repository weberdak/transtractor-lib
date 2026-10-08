//! Rust-native mirror of the Python `Parser` wrapper class.

use crate::configs::db::ConfigDB;
use crate::parsers::flows::layout_to_text_items::layout_to_text_items;
use crate::parsers::flows::pdf_to_text_items::pdf_to_text_items;
use crate::parsers::flows::text_items_to_debug::text_items_to_debug_with_benchmark;
use crate::parsers::flows::text_items_to_layout::text_items_to_layout;
use crate::parsers::flows::text_items_to_statement_data::text_items_to_statement_data_with_benchmark;
use crate::structs::{Benchmark, Spec, StatementData, TextItem};
use pdfsink_rs::PdfDocument;

/// Read a PDF file from disk and extract its text items.
fn pdf_path_to_text_items(pdf_file_path: &str) -> Result<Vec<TextItem>, String> {
    let pdf_document = PdfDocument::open(pdf_file_path)
        .map_err(|e| format!("Failed to open PDF document: {}", e))?;
    pdf_to_text_items(&pdf_document)
        .map_err(|e| format!("Failed to convert PDF to text items: {}", e))
}

/// Read a layout file from disk and parse its text items.
fn layout_path_to_text_items(layout_file_path: &str) -> Result<Vec<TextItem>, String> {
    let layout_str = std::fs::read_to_string(layout_file_path)
        .map_err(|e| format!("Failed to read file at {}: {}", layout_file_path, e))?;
    layout_to_text_items(&layout_str)
}

/// A PDF bank statement parser.
///
/// This parser is initialised with a set of default bank statement extraction
/// configurations. When parsing a PDF, it will attempt to identify applicable
/// configurations based on keywords extracted from the PDF. You can also load
/// custom configurations from JSON files for additional statement formats.
///
/// # Example
///
/// ```no_run
/// use transtractor::parser::Parser;
///
/// let mut parser = Parser::new();
/// parser.load("custom_config.json").unwrap();
/// let statement_data = parser.parse("statement.pdf").unwrap();
/// println!("{}", statement_data);
/// ```
#[derive(Default)]
pub struct Parser {
    db: ConfigDB,
    /// Deprecation warnings emitted by the most recent call to `load`
    last_deprecation_warnings: Vec<String>,
}

impl Parser {
    /// Initialise the Parser with the default configuration database.
    pub fn new() -> Self {
        Self {
            db: ConfigDB::new(),
            last_deprecation_warnings: Vec::new(),
        }
    }

    /// Get deprecation warnings from the last loaded configuration.
    pub fn deprecation_warnings(&self) -> &[String] {
        &self.last_deprecation_warnings
    }

    /// Load a custom parsing configuration from a JSON file.
    ///
    /// Configurations loaded via this method will be registered in the internal
    /// database and will overwrite any existing configuration with the same key.
    pub fn load(&mut self, json_file_path: &str) -> Result<(), String> {
        let json_str = std::fs::read_to_string(json_file_path)
            .map_err(|e| format!("Failed to read file at {}: {}", json_file_path, e))?;

        self.last_deprecation_warnings.clear();
        let warnings = self.db.register_from_str_with_warnings(&json_str)?;
        self.last_deprecation_warnings = warnings;
        Ok(())
    }

    /// Parse the bank statement PDF and return a `StatementData`.
    pub fn parse(&self, pdf_file_path: &str) -> Result<StatementData, String> {
        let mut benchmark = Benchmark::new();
        benchmark.total.start();
        benchmark.pdf_extractor.start();
        let text_items = pdf_path_to_text_items(pdf_file_path)?;
        benchmark.pdf_extractor.pause();
        text_items_to_statement_data_with_benchmark(&self.db, &text_items, &mut benchmark)
    }

    /// Parse the bank statement layout file and return a `StatementData`.
    pub fn parse_layout(&self, layout_file_path: &str) -> Result<StatementData, String> {
        let mut benchmark = Benchmark::new();
        benchmark.total.start();
        benchmark.pdf_extractor.start();
        let text_items = layout_path_to_text_items(layout_file_path)?;
        benchmark.pdf_extractor.pause();
        text_items_to_statement_data_with_benchmark(&self.db, &text_items, &mut benchmark)
    }

    /// Write a summary of the statement data and quality checks for each
    /// statement extraction configuration applied to the PDF.
    pub fn debug(&self, pdf_file_path: &str, output_file: &str) -> Result<(), String> {
        let mut benchmark = Benchmark::new();
        benchmark.total.start();
        benchmark.pdf_extractor.start();
        let text_items = pdf_path_to_text_items(pdf_file_path)?;
        benchmark.pdf_extractor.pause();
        let debug_str = text_items_to_debug_with_benchmark(&self.db, &text_items, &mut benchmark)?;
        write_file(output_file, &debug_str)
    }

    /// Write a summary of the statement data and quality checks for each
    /// statement extraction configuration applied to the layout file.
    pub fn debug_layout(&self, layout_file_path: &str, output_file: &str) -> Result<(), String> {
        let mut benchmark = Benchmark::new();
        benchmark.total.start();
        benchmark.pdf_extractor.start();
        let text_items = layout_path_to_text_items(layout_file_path)?;
        benchmark.pdf_extractor.pause();
        let debug_str = text_items_to_debug_with_benchmark(&self.db, &text_items, &mut benchmark)?;
        write_file(output_file, &debug_str)
    }

    /// Extract and write a text layout representation of the PDF.
    pub fn layout(&self, pdf_file_path: &str, output_file: &str) -> Result<(), String> {
        let text_items = pdf_path_to_text_items(pdf_file_path)?;
        let layout_str = text_items_to_layout(&text_items)?;
        write_file(output_file, &layout_str)
    }

    /// Extract and write a JSON I/O spec representation of the PDF.
    pub fn spec(&self, pdf_file_path: &str, output_file: &str) -> Result<(), String> {
        let text_items = pdf_path_to_text_items(pdf_file_path)?;
        let spec = Spec::new(&self.db, text_items)?;
        let spec_str = spec
            .to_json()
            .map_err(|e| format!("Failed to convert Spec to JSON string: {}", e))?;
        write_file(output_file, &spec_str)
    }

    /// Extract and write a JSON I/O spec representation of the layout file.
    pub fn spec_layout(&self, layout_file_path: &str, output_file: &str) -> Result<(), String> {
        let text_items = layout_path_to_text_items(layout_file_path)?;
        let spec = Spec::new(&self.db, text_items)?;
        let spec_str = spec
            .to_json()
            .map_err(|e| format!("Failed to convert Spec to JSON string: {}", e))?;
        write_file(output_file, &spec_str)
    }

    /// Validate a JSON I/O spec file against the current parser configuration.
    pub fn validate_spec(&self, spec_file_path: &str) -> Result<(), String> {
        let spec_str = std::fs::read_to_string(spec_file_path)
            .map_err(|e| format!("Failed to read file at {}: {}", spec_file_path, e))?;
        let spec = Spec::from_json(&spec_str)
            .map_err(|e| format!("Failed to parse Spec from JSON string: {}", e))?;
        spec.validate(&self.db)
    }
}

/// Helper to write a string to a file at the given path.
fn write_file(output_file: &str, content: &str) -> Result<(), String> {
    std::fs::write(output_file, content)
        .map_err(|e| format!("Failed to write to file at {}: {}", output_file, e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    fn fixtures_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
    }

    fn fixture(name: &str) -> String {
        fixtures_dir().join(name).to_string_lossy().into_owned()
    }

    fn read_fixture(name: &str) -> String {
        std::fs::read_to_string(fixtures_dir().join(name))
            .unwrap_or_else(|e| panic!("Failed to read fixture {}: {}", name, e))
    }

    /// Strip carriage returns so comparisons are stable across CRLF checkouts (e.g. Windows).
    fn normalise_line_endings(content: &str) -> String {
        let mut in_benchmark = false;
        content
            .replace("\r\n", "\n")
            .lines()
            .map(|line| {
                if line == "  Benchmark (microseconds):" {
                    in_benchmark = true;
                    return line.to_string();
                }
                if in_benchmark && line.starts_with("    ") {
                    if let Some((label, _)) = line.split_once(": ") {
                        return format!("{}: <timing>", label);
                    }
                } else {
                    in_benchmark = false;
                }
                line.to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    }

    fn temp_file_path(suffix: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "transtractor_parser_test_{}_{}{}",
            std::process::id(),
            uuid_like(),
            suffix
        ));
        path
    }

    /// Cheap unique-ish suffix without pulling in a UUID dependency.
    fn uuid_like() -> u128 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }

    #[test]
    fn test_load_valid_config() {
        let mut parser = Parser::new();
        let result = parser.load(&fixture("test1_config.json"));
        assert!(result.is_ok(), "Expected load to succeed: {:?}", result);
    }

    #[test]
    fn test_load_raises_error_with_invalid_config() {
        let mut parser = Parser::new();
        let result = parser.load(&fixture("test1_config_invalid.json"));
        assert!(result.is_err(), "Expected load to fail for invalid config");
    }

    #[test]
    fn test_load_emits_deprecation_warnings_for_deprecated_fields() {
        let mut parser = Parser::new();
        parser
            .load(&fixture("test1_config_deprecated.json"))
            .expect("Expected load to succeed for deprecated config");

        let warnings = parser.deprecation_warnings();
        assert_eq!(warnings.len(), 2, "Expected 2 warnings, got {:?}", warnings);
        assert!(warnings[0].contains("fix_text_order"));
        assert!(warnings[0].contains("deprecated since v0.10.0"));
        assert!(warnings[1].contains("transaction_new_line_tol"));
        assert!(warnings[1].contains("deprecated since v0.10.0"));
    }

    #[test]
    fn test_parse_generates_expected_statement_data() {
        let mut parser = Parser::new();
        parser
            .load(&fixture("test1_config.json"))
            .expect("Expected config to load");

        let statement_data = parser
            .parse(&fixture("test1.pdf"))
            .expect("Expected parse to succeed");

        assert_eq!(statement_data.transactions.len(), 62);
        let timings = statement_data.benchmark.as_micros();
        assert!(timings.total > 0, "total benchmark time was not recorded");
        for (stage, elapsed) in [
            (
                "account-number prime",
                timings.parsers_account_number_parser_prime,
            ),
            (
                "account-number parse",
                timings.parsers_account_number_parser_parse,
            ),
            ("start-date prime", timings.parsers_start_date_parser_prime),
            ("start-date parse", timings.parsers_start_date_parser_parse),
            (
                "opening-balance prime",
                timings.parsers_opening_balance_parser_prime,
            ),
            (
                "opening-balance parse",
                timings.parsers_opening_balance_parser_parse,
            ),
            (
                "closing-balance prime",
                timings.parsers_closing_balance_parser_prime,
            ),
            (
                "closing-balance parse",
                timings.parsers_closing_balance_parser_parse,
            ),
            (
                "transaction start prime",
                timings.parsers_transaction_parser_start_prime,
            ),
            (
                "transaction parse",
                timings.parsers_transaction_parser_parse,
            ),
            (
                "transaction stop prime",
                timings.parsers_transaction_parser_stop_prime,
            ),
        ] {
            assert!(elapsed > 0, "{stage} timing was not recorded");
        }
        let display = statement_data.to_string();
        assert!(display.contains("Benchmark (microseconds):"));
        for stage in [
            "Total time",
            "PDF extractor",
            "Tokeniser",
            "Typer",
            "Parsers",
            "Fixers",
            "Checkers",
        ] {
            assert!(display.contains(&format!("    {stage}: ")));
        }
        for stage in [
            "Account number (prime)",
            "Account number (parse)",
            "Start date (prime)",
            "Start date (parse)",
            "Opening balance (prime)",
            "Opening balance (parse)",
            "Closing balance (prime)",
            "Closing balance (parse)",
            "Transaction (prime start)",
            "Transaction (parse)",
            "Transaction (prime stop)",
        ] {
            assert!(display.contains(&format!("        {stage}: ")));
        }
    }

    #[test]
    fn test_parse_generates_expected_statement_data_with_start_date_first_match() {
        let mut parser = Parser::new();
        parser
            .load(&fixture("test1a_config.json"))
            .expect("Expected config to load");

        let statement_data = parser
            .parse(&fixture("test1a.pdf"))
            .expect("Expected parse to succeed");

        assert_eq!(statement_data.transactions.len(), 62);
    }

    #[test]
    fn test_parse_falls_back_to_next_config_if_first_fails() {
        let mut parser = Parser::new();
        parser
            .load(&fixture("test1_config.json"))
            .expect("Expected config to load");
        parser
            .load(&fixture("test1a_config.json"))
            .expect("Expected config to load");

        let statement_data = parser
            .parse(&fixture("test1a.pdf"))
            .expect("Expected parse to succeed");

        assert_eq!(statement_data.transactions.len(), 62);
    }

    #[test]
    fn test_parse_raises_error_without_config() {
        let parser = Parser::new();
        let result = parser.parse(&fixture("test1.pdf"));
        assert!(result.is_err(), "Expected parse to fail without a config");
    }

    #[test]
    fn test_parse_raises_error_with_misconfigured_config() {
        let mut parser = Parser::new();
        parser
            .load(&fixture("test1_config_misconfigured.json"))
            .expect("Expected misconfigured config to load");

        let result = parser.parse(&fixture("test1.pdf"));
        assert!(
            result.is_err(),
            "Expected parse to fail with a misconfigured config"
        );
    }

    #[test]
    fn test_parse_layout_generates_expected_statement_data() {
        let mut parser = Parser::new();
        parser
            .load(&fixture("test1_config.json"))
            .expect("Expected config to load");

        let statement_data = parser
            .parse_layout(&fixture("test1_layout.txt"))
            .expect("Expected parse_layout to succeed");

        assert_eq!(statement_data.transactions.len(), 62);
    }

    #[test]
    fn test_parse_layout_raises_error_without_config() {
        let parser = Parser::new();
        let result = parser.parse_layout(&fixture("test1_layout.txt"));
        assert!(
            result.is_err(),
            "Expected parse_layout to fail without a config"
        );
    }

    #[test]
    fn test_parse_layout_raises_error_with_misconfigured_config() {
        let mut parser = Parser::new();
        parser
            .load(&fixture("test1_config_misconfigured.json"))
            .expect("Expected misconfigured config to load");

        let result = parser.parse_layout(&fixture("test1_layout.txt"));
        assert!(
            result.is_err(),
            "Expected parse_layout to fail with a misconfigured config"
        );
    }

    #[test]
    fn test_debug_generates_correct_output() {
        let mut parser = Parser::new();
        parser
            .load(&fixture("test1_config.json"))
            .expect("Expected config to load");

        let output_path = temp_file_path(".txt");
        let output_path_str = output_path.to_string_lossy().into_owned();

        parser
            .debug(&fixture("test1.pdf"), &output_path_str)
            .expect("Expected debug to succeed");

        let generated = std::fs::read_to_string(&output_path).unwrap();
        let expected = read_fixture("test1_debug.txt");
        std::fs::remove_file(&output_path).ok();

        assert_eq!(
            normalise_line_endings(&generated),
            normalise_line_endings(&expected)
        );
    }

    #[test]
    fn test_debug_raises_error_without_config() {
        let parser = Parser::new();
        let output_path = temp_file_path(".txt");
        let result = parser.debug(&fixture("test1.pdf"), &output_path.to_string_lossy());
        assert!(result.is_err(), "Expected debug to fail without a config");
    }

    #[test]
    fn test_debug_layout_generates_correct_output() {
        let mut parser = Parser::new();
        parser
            .load(&fixture("test1_config.json"))
            .expect("Expected config to load");

        let output_path = temp_file_path(".txt");
        let output_path_str = output_path.to_string_lossy().into_owned();

        parser
            .debug_layout(&fixture("test1_layout.txt"), &output_path_str)
            .expect("Expected debug_layout to succeed");

        let generated = std::fs::read_to_string(&output_path).unwrap();
        let expected = read_fixture("test1_debug_layout.txt");
        std::fs::remove_file(&output_path).ok();

        assert_eq!(
            normalise_line_endings(&generated),
            normalise_line_endings(&expected)
        );
    }

    #[test]
    fn test_debug_layout_raises_error_without_config() {
        let parser = Parser::new();
        let output_path = temp_file_path(".txt");
        let result =
            parser.debug_layout(&fixture("test1_layout.txt"), &output_path.to_string_lossy());
        assert!(
            result.is_err(),
            "Expected debug_layout to fail without a config"
        );
    }

    #[test]
    fn test_layout_generates_correct_output() {
        let parser = Parser::new();
        let output_path = temp_file_path(".txt");
        let output_path_str = output_path.to_string_lossy().into_owned();

        parser
            .layout(&fixture("test1.pdf"), &output_path_str)
            .expect("Expected layout to succeed");

        let generated = std::fs::read_to_string(&output_path).unwrap();
        let expected = read_fixture("test1_layout.txt");
        std::fs::remove_file(&output_path).ok();

        assert_eq!(
            normalise_line_endings(&generated),
            normalise_line_endings(&expected)
        );
    }

    #[test]
    fn test_spec_generates_correct_output() {
        let mut parser = Parser::new();
        parser
            .load(&fixture("test1_config.json"))
            .expect("Expected config to load");

        let output_path = temp_file_path(".json");
        let output_path_str = output_path.to_string_lossy().into_owned();

        parser
            .spec(&fixture("test1.pdf"), &output_path_str)
            .expect("Expected spec to succeed");

        let generated = std::fs::read_to_string(&output_path).unwrap();
        let expected = read_fixture("test1_spec.json");
        std::fs::remove_file(&output_path).ok();

        assert_eq!(
            normalise_line_endings(&generated),
            normalise_line_endings(&expected)
        );
    }

    #[test]
    fn test_spec_raises_error_without_config() {
        let parser = Parser::new();
        let output_path = temp_file_path(".json");
        let result = parser.spec(&fixture("test1.pdf"), &output_path.to_string_lossy());
        assert!(result.is_err(), "Expected spec to fail without a config");
    }

    #[test]
    fn test_spec_raises_error_with_misconfigured_config() {
        let mut parser = Parser::new();
        parser
            .load(&fixture("test1_config_misconfigured.json"))
            .expect("Expected misconfigured config to load");

        let output_path = temp_file_path(".json");
        let result = parser.spec(&fixture("test1.pdf"), &output_path.to_string_lossy());
        assert!(
            result.is_err(),
            "Expected spec to fail with a misconfigured config"
        );
    }

    #[test]
    fn test_spec_layout_generates_correct_output() {
        let mut parser = Parser::new();
        parser
            .load(&fixture("test1_config.json"))
            .expect("Expected config to load");

        let output_path = temp_file_path(".json");
        let output_path_str = output_path.to_string_lossy().into_owned();

        parser
            .spec_layout(&fixture("test1_layout.txt"), &output_path_str)
            .expect("Expected spec_layout to succeed");

        let generated = std::fs::read_to_string(&output_path).unwrap();
        let expected = read_fixture("test1_spec.json");
        std::fs::remove_file(&output_path).ok();

        assert_eq!(
            normalise_line_endings(&generated),
            normalise_line_endings(&expected)
        );
    }

    #[test]
    fn test_spec_layout_raises_error_without_config() {
        let parser = Parser::new();
        let output_path = temp_file_path(".json");
        let result =
            parser.spec_layout(&fixture("test1_layout.txt"), &output_path.to_string_lossy());
        assert!(
            result.is_err(),
            "Expected spec_layout to fail without a config"
        );
    }

    #[test]
    fn test_validate_spec_with_valid_spec_file() {
        let mut parser = Parser::new();
        parser
            .load(&fixture("test1_config.json"))
            .expect("Expected config to load");

        let result = parser.validate_spec(&fixture("test1_spec.json"));
        assert!(
            result.is_ok(),
            "Expected valid spec to validate: {:?}",
            result
        );
    }

    #[test]
    fn test_validate_spec_with_invalid_spec_file() {
        let mut parser = Parser::new();
        parser
            .load(&fixture("test1_config.json"))
            .expect("Expected config to load");

        let result = parser.validate_spec(&fixture("test1_spec_invalid.json"));
        assert!(result.is_err(), "Expected invalid spec to fail validation");
    }

    #[test]
    fn test_validate_spec_raises_error_without_config() {
        let parser = Parser::new();
        let result = parser.validate_spec(&fixture("test1_spec.json"));
        assert!(
            result.is_err(),
            "Expected validate_spec to fail without a config"
        );
    }

    /// Load test1 config with an unmatchable closing balance term and the flag set.
    fn parse_test1_layout_with_flag(flag: bool) -> crate::structs::AccountData {
        let json = read_fixture("test1_config.json")
            .replace("\"Closing balance:\"", "\"No such term:\"")
            .replace(
                "[\"date\", \"description\", \"amount\"],\n        [\"description\", \"amount\", \"balance\"],\n        [\"description\", \"amount\"]",
                "[\"description\", \"amount\", \"balance\"]",
            )
            .replace(
                "\"closing_balance_invert\": false,",
                &format!(
                    "\"closing_balance_invert\": false,\n    \"closing_balance_set_from_last_transaction\": {flag},"
                ),
            );
        let path = temp_file_path(".json");
        std::fs::write(&path, json).unwrap();
        let mut parser = Parser::new();
        parser.load(&path.to_string_lossy()).unwrap();
        std::fs::remove_file(&path).ok();

        let items = layout_path_to_text_items(&fixture("test1_layout.txt")).unwrap();
        let configs = parser.db.identify(&items);
        let tokenised = crate::structs::text_items::tokenise_items(&items);
        crate::parsers::top::parse_text_items(&configs[0], &tokenised)
    }

    #[test]
    fn test_closing_balance_set_from_last_transaction() {
        let off = parse_test1_layout_with_flag(false);
        assert_eq!(off.closing_balance, None);

        let on = parse_test1_layout_with_flag(true);
        assert_eq!(on.closing_balance, Some(11663.82));
    }
}

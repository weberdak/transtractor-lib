"""Tests for the Parser parse_layout method."""

import tempfile
from pathlib import Path

import pytest
from transtractor import ParseError
from transtractor.parser import Parser
from transtractor.structs.statement_data import StatementData


def test_parse_layout_generates_correct_csv():
    """Test that parsing a layout file generates a CSV matching the expected output."""
    parser = Parser()

    # Parse the test layout file
    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    test_layout = fixtures_dir / "test1_layout.txt"
    config = fixtures_dir / "test1_config.json"
    parser.load(str(config))
    expected_csv = fixtures_dir / "test1_parsed.csv"

    statement_data: StatementData = parser.parse_layout(str(test_layout))
    assert all(
        isinstance(getattr(statement_data.benchmark, stage), int)
        for stage in (
            "pdf_extractor",
            "total",
            "tokeniser",
            "typer",
            "parsers",
            "parsers_account_number_parser_prime",
            "parsers_account_number_parser_parse",
            "parsers_start_date_parser_prime",
            "parsers_start_date_parser_parse",
            "parsers_opening_balance_parser_prime",
            "parsers_opening_balance_parser_parse",
            "parsers_closing_balance_parser_prime",
            "parsers_closing_balance_parser_parse",
            "parsers_transaction_parser_start_prime",
            "parsers_transaction_parser_parse",
            "parsers_transaction_parser_stop_prime",
            "fixers",
            "checkers",
        )
    )

    assert {
        transaction.account_number for transaction in statement_data.transactions
    } == {"1234 5678 9123 4567"}

    # Generate CSV in a temporary file
    with tempfile.NamedTemporaryFile(
        mode="w", suffix=".csv", delete=False, newline=""
    ) as tmp_file:
        tmp_csv_path = tmp_file.name
        statement_data.to_csv(tmp_csv_path)

    try:
        # Read both CSV files
        with open(tmp_csv_path, encoding="utf-8") as generated:
            generated_lines = generated.readlines()

        with open(expected_csv, encoding="utf-8") as expected:
            expected_lines = expected.readlines()

        # Compare line by line
        assert len(generated_lines) == len(expected_lines), (
            f"CSV line count mismatch: generated {len(generated_lines)} lines, "
            f"expected {len(expected_lines)} lines"
        )

        for i, (generated_line, expected_line) in enumerate(
            zip(generated_lines, expected_lines, strict=True)
        ):
            assert generated_line == expected_line, (
                f"CSV content mismatch at line {i + 1}:\n"
                f"Generated: {generated_line}\n"
                f"Expected: {expected_line}"
            )
    finally:
        # Clean up temporary file
        Path(tmp_csv_path).unlink(missing_ok=True)


def test_parse_layout_parser_error_without_config():
    """Test that parsing a layout file without loading a config raises
    ParseError."""
    parser = Parser()

    # Parse the test layout file without loading the config file
    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    test_layout = fixtures_dir / "test1_layout.txt"

    # Should raise ParseError since no config is loaded
    with pytest.raises(ParseError):
        parser.parse_layout(str(test_layout))


def test_parse_layout_raises_parser_error_with_misconfigured_config():
    """Test that parsing a layout file with a misconfigured config raises
    ParseError."""
    parser = Parser()

    # Load the misconfigured config file
    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    test_layout = fixtures_dir / "test1_layout.txt"
    misconfigured_config = fixtures_dir / "test1_config_misconfigured.json"
    parser.load(str(misconfigured_config))

    # Should raise ParseError since the config is misconfigured
    with pytest.raises(ParseError):
        parser.parse_layout(str(test_layout))

"""Tests for the Parser test method."""

import csv
import tempfile
from pathlib import Path

import pytest
from transtractor.parser import Parser


def normalize_csv_for_comparison(csv_path: str, fixtures_dir: Path) -> list[list[str]]:
    """Read CSV and normalize by zeroing out timing columns and converting paths to
    relative."""
    with open(csv_path, encoding="utf-8") as f:
        reader = csv.reader(f)
        rows = list(reader)

    # tests_dir is the parent of fixtures_dir (e.g., /path/to/tests)
    tests_dir = fixtures_dir.parent
    # project_root is the parent of tests_dir (e.g., /path/to/project)
    project_root = tests_dir.parent

    # Zero out timing columns (indices 4, 5, 6, 7 based on header:
    # "PDF File", "Pages", "Transactions",
    # "Total Time (ms)",
    # "Status", "Reason Failed"
    for i, row in enumerate(rows):
        if i == 0 or not row:  # Skip header and empty rows
            continue
        # Convert absolute path to relative path from project root
        if row[0]:  # PDF File column
            pdf_path = Path(row[0])
            try:
                # Make path relative to project root to get
                # "tests/fixtures/test1.pdf"
                row[0] = pdf_path.relative_to(project_root).as_posix()
            except ValueError:
                # Path is not relative to project_root, keep as is
                pass

        # Zero out timing columns if they contain non-zero values
        row[2] = "0"

    return rows


def test_test_generates_correct_csv_with_valid_config():
    """Test that parser.test with valid config matches test1_test.csv."""
    parser = Parser()

    # Load valid config
    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    config = fixtures_dir / "test1_config.json"
    config_1a = fixtures_dir / "test1a_config.json"
    parser.load(str(config))
    parser.load(str(config_1a))
    expected_csv = fixtures_dir / "test1_test.csv"

    # Generate test output in a temporary file
    with tempfile.NamedTemporaryFile(
        mode="w", suffix=".csv", delete=False, newline=""
    ) as tmp_file:
        tmp_csv_path = tmp_file.name

    try:
        parser.test(str(fixtures_dir), output_file=tmp_csv_path, log_level="WARNING")

        # Read and normalize both CSV files
        generated_rows = normalize_csv_for_comparison(tmp_csv_path, fixtures_dir)
        expected_rows = normalize_csv_for_comparison(str(expected_csv), fixtures_dir)

        # Compare
        assert len(generated_rows) == len(expected_rows), (
            f"CSV row count mismatch: generated {len(generated_rows)} rows, "
            f"expected {len(expected_rows)} rows"
        )

        for i, (generated_row, expected_row) in enumerate(
            zip(generated_rows, expected_rows, strict=True)
        ):
            assert generated_row == expected_row, (
                f"CSV content mismatch at row {i + 1}:\n"
                f"Generated: {generated_row}\n"
                f"Expected: {expected_row}"
            )
    finally:
        # Clean up temporary file
        Path(tmp_csv_path).unlink(missing_ok=True)


def test_test_generates_correct_csv_with_misconfigured_config():
    """Test that parser.test with misconfigured config matches
    test1_test_NoErrorFreeStatementData.csv."""
    parser = Parser()

    # Load misconfigured config
    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    misconfigured_config = fixtures_dir / "test1_config_misconfigured.json"
    parser.load(str(misconfigured_config))
    expected_csv = fixtures_dir / "test1_test_NoErrorFreeStatementData.csv"

    # Generate test output in a temporary file
    with tempfile.NamedTemporaryFile(
        mode="w", suffix=".csv", delete=False, newline=""
    ) as tmp_file:
        tmp_csv_path = tmp_file.name

    try:
        parser.test(str(fixtures_dir), output_file=tmp_csv_path, log_level="WARNING")

        # Read and normalize both CSV files
        generated_rows = normalize_csv_for_comparison(tmp_csv_path, fixtures_dir)
        expected_rows = normalize_csv_for_comparison(str(expected_csv), fixtures_dir)

        # Compare
        assert len(generated_rows) == len(expected_rows), (
            f"CSV row count mismatch: generated {len(generated_rows)} rows, "
            f"expected {len(expected_rows)} rows"
        )

        for i, (generated_row, expected_row) in enumerate(
            zip(generated_rows, expected_rows, strict=True)
        ):
            assert generated_row == expected_row, (
                f"CSV content mismatch at row {i + 1}:\n"
                f"Generated: {generated_row}\n"
                f"Expected: {expected_row}"
            )
    finally:
        # Clean up temporary file
        Path(tmp_csv_path).unlink(missing_ok=True)


def test_test_generates_correct_csv_without_config():
    """Test that parser.test without config matches
    test1_test_StatementNotSupported.csv."""
    parser = Parser()

    # Don't load any config
    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    expected_csv = fixtures_dir / "test1_test_StatementNotSupported.csv"

    # Generate test output in a temporary file
    with tempfile.NamedTemporaryFile(
        mode="w", suffix=".csv", delete=False, newline=""
    ) as tmp_file:
        tmp_csv_path = tmp_file.name

    try:
        parser.test(str(fixtures_dir), output_file=tmp_csv_path, log_level="WARNING")

        # Read and normalize both CSV files
        generated_rows = normalize_csv_for_comparison(tmp_csv_path, fixtures_dir)
        expected_rows = normalize_csv_for_comparison(str(expected_csv), fixtures_dir)

        # Compare
        assert len(generated_rows) == len(expected_rows), (
            f"CSV row count mismatch: generated {len(generated_rows)} rows, "
            f"expected {len(expected_rows)} rows"
        )

        for i, (generated_row, expected_row) in enumerate(
            zip(generated_rows, expected_rows, strict=True)
        ):
            assert generated_row == expected_row, (
                f"CSV content mismatch at row {i + 1}:\n"
                f"Generated: {generated_row}\n"
                f"Expected: {expected_row}"
            )
    finally:
        # Clean up temporary file
        Path(tmp_csv_path).unlink(missing_ok=True)


def test_test_raises_value_error_with_invalid_log_level():
    """Test that parser.test with an invalid log_level raises ValueError."""
    parser = Parser()

    fixtures_dir = Path(__file__).parent.parent / "fixtures"

    # Generate test output in a temporary file
    with tempfile.NamedTemporaryFile(
        mode="w", suffix=".csv", delete=False, newline=""
    ) as tmp_file:
        tmp_csv_path = tmp_file.name

    try:
        # Should raise ValueError for invalid log level
        with pytest.raises(ValueError):
            parser.test(
                str(fixtures_dir), output_file=tmp_csv_path, log_level="INVALID"
            )
    finally:
        # Clean up temporary file
        Path(tmp_csv_path).unlink(missing_ok=True)

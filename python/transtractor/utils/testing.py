"""Testing utilities for the Transtractor parser module."""

import csv
import logging
import time
from pathlib import Path
from typing import TYPE_CHECKING, cast

from ..structs.statement_data import StatementData

if TYPE_CHECKING:
    from ..parser import Parser


class TestData:
    """Class for managing test data files and directories."""

    def __init__(self, pdf_file_path: str, parser: "Parser"):
        self.pdf_file_path = pdf_file_path  # The PDF file being tested
        self.parser = parser  # The Parser instance used for testing
        self.num_transactions: int = 0  # Number of transactions extracted
        self.total_time: int = 0  # Total time taken for the test in ms
        self.status: str = ""  # Status of the test (PASS/FAIL)
        self.reason_failed: str = ""  # Error message if any

    @staticmethod
    def get_header_all() -> list[str]:
        """Get all headers for writing CSV file."""
        return [
            "PDF File",
            "Transactions",
            "Total Time (ms)",
            "Status",
            "Reason Failed",
        ]

    @staticmethod
    def get_header_log() -> list[str]:
        """Get headers for writing log file."""
        return [
            "Status",
            "Transactions",
            "Total Time (ms)",
            "PDF File",
        ]

    def get_all(self) -> list[str]:
        """Get all data fields as strings for writing CSV file."""
        return [
            Path(self.pdf_file_path).as_posix(),
            str(self.num_transactions),
            str(self.total_time),
            self.status,
            self.reason_failed,
        ]

    def get_log(self) -> list[str]:
        """Get data fields as strings for writing log file."""
        return [
            self.status,
            str(self.num_transactions),
            str(self.total_time),
            Path(self.pdf_file_path).as_posix(),
        ]

    def run(self) -> None:
        """Run the test on the PDF file using the provided parser."""
        start_total = time.time()

        # Try to parse the statement
        try:
            sd: StatementData = cast(
                StatementData,
                self.parser.parse(self.pdf_file_path),
            )
            self.num_transactions = len(sd.transactions)
            self.status = "PASS"
        except Exception as e:
            self.status = "FAIL"
            self.reason_failed = str(e)

        end_total = time.time()
        self.total_time = int((end_total - start_total) * 1000)


def run_test_protocol(
    pdf_dir: str, parser: "Parser", output_file: str = "", log_level: str = "INFO"
) -> list[TestData]:
    """Run test protocol on all PDFs in a given directory and sub-directories.

    :param pdf_dir: Path to the directory containing PDF files to be tested
    :param parser: Parser instance to use for testing
    :param output_file: Optional path to output CSV file for test results
    :param log_level: Logging level (DEBUG, INFO, WARNING, ERROR, CRITICAL)

    Note: Set log_level to "WARNING" or higher to suppress terminal output.

    :raises NotADirectoryError: If pdf_dir does not exist or is not a directory
    """
    if not Path(pdf_dir).is_dir():
        raise NotADirectoryError(f"Input directory does not exist: {pdf_dir}")

    # Set up logging
    numeric_level = getattr(logging, log_level.upper(), None)
    if not isinstance(numeric_level, int):
        raise ValueError(f"Invalid log level: {log_level}")
    logging.basicConfig(level=numeric_level, format="%(message)s")
    logger = logging.getLogger()

    # Get all PDF files in the directory and sub-directories
    pdf_files: list[str] = sorted(str(p) for p in Path(pdf_dir).rglob("*.pdf"))
    num_files = len(pdf_files)
    num_passed = 0
    num_failed = 0
    test_results: list[TestData] = []
    log_header = "\t".join(["Test"] + TestData.get_header_log())
    logger.info(log_header)

    file_count = 1
    for pdf_file in pdf_files:
        test_data = TestData(pdf_file, parser)
        test_data.run()
        test_results.append(test_data)
        logger.info(
            "%s/%s\t%s\t%s\t%sms\t%s",
            file_count,
            num_files,
            test_data.status,
            test_data.num_transactions,
            test_data.total_time,
            test_data.pdf_file_path,
        )
        if test_data.status == "PASS":
            num_passed += 1
        else:
            num_failed += 1
        file_count += 1

    # Write results to output CSV file if specified
    if output_file:
        with open(output_file, mode="w", newline="", encoding="utf-8") as csvfile:
            writer = csv.writer(csvfile)
            writer.writerow(TestData.get_header_all())
            for result in test_results:
                writer.writerow(result.get_all())

    logger.info(
        "Summary: %s passed, %s failed out of %s files.",
        num_passed,
        num_failed,
        num_files,
    )

    return test_results

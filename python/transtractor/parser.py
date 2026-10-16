"""Python wrapper for the Transtractor PDF bank statement parser."""

import warnings
from typing import cast

from .structs.statement_data import StatementData
from .transtractor import LibParser
from .utils.testing import run_test_protocol


class Parser:
    """A PDF bank statement parser.

    This parser will be initialised with a set of default bank statement
    extraction configurations. When parsing a PDF, it will attempt to identify
    applicable configurations based on keywords extracted from the PDF. You
    can also load custom configurations from JSON files for additional statement
    formats.

    Example:
        parser = Parser()
        parser.load('custom_config.json')
        statement_data = parser.parse('statement.pdf')
        print(statement_data)
        statement_data.to_csv('output.csv')
    """

    def __init__(self):
        """Initialise the Parser with default database."""
        self._inner = LibParser()

    def parse(self, pdf_file_path: str) -> StatementData:
        """Parse the bank statement PDF and return a StatementData object.

        :param pdf_file_path: Path to the PDF file to be processed
        :return: StatementData object representing the parsed bank statement data
        :raises ParseError: If statement is not recognisable or not parsed correctly
        """
        sd: StatementData = cast(
            StatementData, self._inner.py_pdf_path_to_py_statement_data(pdf_file_path)
        )
        sd.filename = pdf_file_path
        return sd

    def parse_layout(self, layout_file_path: str) -> StatementData:
        """Parse the bank statement layout string and return a StatementData object.

        :param layout_file_path: Path to the layout file to be processed
        :return: StatementData object representing the parsed bank statement data
        :raises ParseError: If statement is not recognisable or not parsed correctly
        """
        sd: StatementData = cast(
            StatementData,
            self._inner.py_layout_path_to_py_statement_data(layout_file_path),
        )
        sd.filename = layout_file_path
        return sd

    def debug(self, pdf_file_path: str, output_file: str):
        """Write a summary of the statement data and quality checks for
        each statement extraction configuration applied.

        :param pdf_file_path: Path to the PDF file to be processed
        :param output_file: Path to the output debug text file
        :raises ParseError: If statement is not recognisable or not parsed correctly
        """
        self._inner.py_pdf_path_to_debug(pdf_file_path, output_file)

    def debug_layout(self, layout_file_path: str, output_file: str):
        """Write a summary of the statement data and quality checks for
        each statement extraction configuration applied.

        :param layout_file_path: Path to the layout file to be processed
        :param output_file: Path to the output debug text file
        :raises ParseError: If statement is not recognisable or not parsed correctly
        """
        self._inner.py_layout_path_to_debug(layout_file_path, output_file)

    def layout(self, pdf_file_path: str, output_file: str) -> None:
        """Extract, write and return a text layout representation of the PDF page.

        :param pdf_file_path: Path to the PDF file to be processed
        """
        self._inner.py_pdf_path_to_layout(pdf_file_path, output_file)

    def load(self, json_file_path: str) -> None:
        """Load a custom parsing configuration from a JSON file.

        Configurations loaded via this method will be registered in the
        internal database and will overwrite any existing configuration with the same
        key.

        :param json_file_path: Path to the JSON configuration file
        :raises ConfigLoadError: Configuration file is invalid or cannot be loaded

        See the docs for detailed instructions for creating custom
        configuration JSON files.
        """
        # Register the configuration via string to capture deprecation warnings
        self._inner.register_config_from_json(json_file_path)

        # Check for and emit any deprecation warnings
        deprecation_warnings = self._inner.get_deprecation_warnings()
        for warning_msg in deprecation_warnings:
            warnings.warn(
                f"Configuration from {json_file_path} uses deprecated field: "
                f"{warning_msg}",
                DeprecationWarning,
                stacklevel=2,
            )

    def spec(self, pdf_file_path: str, output_file: str) -> None:
        """Extract and write a JSON I/O spec representation of a PDF file.

        :param pdf_file_path: Path to the PDF file to be processed
        :param output_file: Path to the output JSON spec file
        :raises ParseError: If statement is not recognisable or not parsed correctly
        """
        self._inner.py_pdf_path_to_spec(pdf_file_path, output_file)

    def spec_layout(self, layout_file_path: str, output_file: str) -> None:
        """Extract and write a JSON I/O spec representation of a layout text file.

        :param layout_file_path: Path to the layout text file to be processed
        :param output_file: Path to the output JSON spec file
        :raises ParseError: If statement is not recognisable or not parsed correctly
        """
        self._inner.py_layout_path_to_spec(layout_file_path, output_file)

    def validate_spec(self, spec_file_path: str) -> None:
        """Validate a JSON I/O spec file against the current parser configuration.

        :param spec_file_path: Path to the JSON spec file to be validated
        :raises SpecError: If the TextItems in the spec file cannot be parsed, or
            parsed differently to what is expected in the spec file's StatementData.
        """
        self._inner.py_spec_path_to_validate(spec_file_path)

    def test(
        self, pdf_dir: str, output_file: str = "", log_level: str = "INFO"
    ) -> None:
        """Try to parse all PDFs in a given directory and sub-directories
        using the current parser configuration database. Optionally outputs
        a CSV file summarising the test results.

        :param pdf_dir: Path to the directory containing PDF files to be tested
        :param output_file: Optional path to output CSV file for test results
        :param log_level: Logging level for test output (e.g., "INFO", "WARNING")
        :return: None

        Note: Set log_level to "WARNING" or higher to suppress terminal output.
        """
        run_test_protocol(pdf_dir, self, output_file, log_level)

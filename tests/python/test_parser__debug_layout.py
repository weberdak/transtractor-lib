"""Tests for the Parser debug_layout functionality."""

import tempfile
from pathlib import Path

import pytest
from transtractor import ParseError
from transtractor.parser import Parser


def normalise_benchmark_timings(content: str) -> str:
    in_benchmark = False
    normalized = []
    for line in content.replace("\r\n", "\n").splitlines():
        if line in {"  Benchmark (microseconds):", "Benchmark (microseconds):"}:
            in_benchmark = True
            normalized.append(line)
        elif in_benchmark and line.startswith(" ") and ": " in line:
            label, _ = line.split(": ", maxsplit=1)
            normalized.append(f"{label}: <TIMING>")
        else:
            in_benchmark = False
            normalized.append(line)
    return "\n".join(normalized) + "\n"


def test_debug_layout_generates_correct_output():
    """Test that debug_layout for test1_layout.txt generates output matching
    test1_debug_layout.txt."""
    parser = Parser()

    # Parse the test layout file
    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    test_layout = fixtures_dir / "test1_layout.txt"
    config = fixtures_dir / "test1_config.json"
    parser.load(str(config))
    expected_debug = fixtures_dir / "test1_debug_layout.txt"

    # Generate debug output in a temporary file
    with tempfile.NamedTemporaryFile(mode="w", suffix=".txt", delete=False) as tmp_file:
        tmp_debug_path = tmp_file.name

    try:
        parser.debug_layout(str(test_layout), tmp_debug_path)

        # Read both debug files
        with open(tmp_debug_path, encoding="utf-8") as generated:
            generated_content = generated.read()

        with open(expected_debug, encoding="utf-8") as expected:
            expected_content = expected.read()

        # Compare content
        assert normalise_benchmark_timings(
            generated_content
        ) == normalise_benchmark_timings(expected_content), (
            "Debug output mismatch:\n"
            f"Generated length: {len(generated_content)}\n"
            f"Expected length: {len(expected_content)}"
        )
    finally:
        # Clean up temporary file
        Path(tmp_debug_path).unlink(missing_ok=True)


def test_debug_raises_parse_error_without_config():
    """Test that debug without loading a config raises ParseError."""
    parser = Parser()

    # Try to debug the test layout file without loading the config file
    fixtures_dir = Path(__file__).parent.parent / "fixtures"
    test_layout = fixtures_dir / "test1_layout.txt"

    with tempfile.NamedTemporaryFile(mode="w", suffix=".txt", delete=False) as tmp_file:
        tmp_debug_path = tmp_file.name

    try:
        # Should raise ParseError since no config is loaded
        with pytest.raises(ParseError):
            parser.debug_layout(str(test_layout), tmp_debug_path)
    finally:
        # Clean up temporary file
        Path(tmp_debug_path).unlink(missing_ok=True)

"""transtractor package initializer."""

from .parser import Parser
from .structs.statement_data import StatementData
from .structs.transaction import Transaction
from .transtractor import (
    LibParser,  # Rust PyO3 class
    ParseError,  # Rust PyO3 class
    SpecError,  # Rust PyO3 class
)

__all__ = [
    "LibParser",
    "Parser",
    "ParseError",
    "SpecError",
    "StatementData",
    "Transaction",
]

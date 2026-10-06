export interface Transaction {
  date: number;
  index: number;
  description: string;
  amount: number;
  balance: number;
  account_number: string;
}

export interface StatementData {
  transactions: Transaction[];
  benchmark: Benchmark;
}

export interface Benchmark {
  /** Stage durations in microseconds. */
  total: bigint;
  pdf_extractor: bigint;
  tokeniser: bigint;
  typer: bigint;
  parsers: bigint;
  parsers_account_number_parser_prime: bigint;
  parsers_account_number_parser_parse: bigint;
  parsers_start_date_parser_prime: bigint;
  parsers_start_date_parser_parse: bigint;
  parsers_opening_balance_parser_prime: bigint;
  parsers_opening_balance_parser_parse: bigint;
  parsers_closing_balance_parser_prime: bigint;
  parsers_closing_balance_parser_parse: bigint;
  parsers_transaction_parser_start_prime: bigint;
  parsers_transaction_parser_parse: bigint;
  parsers_transaction_parser_stop_prime: bigint;
  fixers: bigint;
  checkers: bigint;
}

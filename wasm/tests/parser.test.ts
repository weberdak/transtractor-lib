import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import { beforeAll, describe, expect, it } from "vitest";

import { Parser } from "../src/index.js";

const FIXTURE_DIR = resolve(import.meta.dirname, "../../tests/fixtures");

describe("Parser", () => {
  let parser: Parser;

  beforeAll(async () => {
    parser = await Parser.create();
    const configPath = resolve(FIXTURE_DIR, "test1_config.json");
    const configJson = readFileSync(configPath, "utf8");
    parser.loadConfigFromJson(configJson);
  });

  it("parses layout text into statement data", () => {
    const layoutPath = resolve(FIXTURE_DIR, "test1_layout.txt");
    const specPath = resolve(FIXTURE_DIR, "test1_spec.json");

    const layoutText = readFileSync(layoutPath, "utf8");
    const expected = JSON.parse(readFileSync(specPath, "utf8")) as {
      statement_data: {
        transactions: unknown[];
      };
    };

    const actual = parser.parseLayoutText(layoutText);

    expect(actual.benchmark).toMatchObject({
      total: expect.any(BigInt),
      pdf_extractor: expect.any(BigInt),
      tokeniser: expect.any(BigInt),
      typer: expect.any(BigInt),
      parsers: expect.any(BigInt),
      parsers_account_number_parser_prime: expect.any(BigInt),
      parsers_account_number_parser_parse: expect.any(BigInt),
      parsers_start_date_parser_prime: expect.any(BigInt),
      parsers_start_date_parser_parse: expect.any(BigInt),
      parsers_opening_balance_parser_prime: expect.any(BigInt),
      parsers_opening_balance_parser_parse: expect.any(BigInt),
      parsers_closing_balance_parser_prime: expect.any(BigInt),
      parsers_closing_balance_parser_parse: expect.any(BigInt),
      parsers_transaction_parser_start_prime: expect.any(BigInt),
      parsers_transaction_parser_parse: expect.any(BigInt),
      parsers_transaction_parser_stop_prime: expect.any(BigInt),
      fixers: expect.any(BigInt),
      checkers: expect.any(BigInt),
    });

    expect(actual.transactions).toHaveLength(
      expected.statement_data.transactions.length,
    );
    expect(actual.transactions[0]).toMatchObject({
      date: expect.any(Number),
      index: expect.any(Number),
      description: expect.any(String),
      amount: expect.any(Number),
      balance: expect.any(Number),
      account_number: expect.any(String),
    });
  });

  it("parses PDF bytes into statement data", () => {
    const pdfPath = resolve(FIXTURE_DIR, "test1.pdf");
    const specPath = resolve(FIXTURE_DIR, "test1_spec.json");

    const pdfBytes = new Uint8Array(readFileSync(pdfPath));
    const expected = JSON.parse(readFileSync(specPath, "utf8")) as {
      statement_data: {
        key: string;
        account_number: string;
        start_date: number;
        opening_balance: number;
        closing_balance: number;
        transactions: unknown[];
      };
    };

    const actual = parser.parseBytes(pdfBytes);

    expect(actual.benchmark).toMatchObject({
      total: expect.any(BigInt),
      pdf_extractor: expect.any(BigInt),
      tokeniser: expect.any(BigInt),
      typer: expect.any(BigInt),
      parsers: expect.any(BigInt),
      parsers_account_number_parser_prime: expect.any(BigInt),
      parsers_account_number_parser_parse: expect.any(BigInt),
      parsers_start_date_parser_prime: expect.any(BigInt),
      parsers_start_date_parser_parse: expect.any(BigInt),
      parsers_opening_balance_parser_prime: expect.any(BigInt),
      parsers_opening_balance_parser_parse: expect.any(BigInt),
      parsers_closing_balance_parser_prime: expect.any(BigInt),
      parsers_closing_balance_parser_parse: expect.any(BigInt),
      parsers_transaction_parser_start_prime: expect.any(BigInt),
      parsers_transaction_parser_parse: expect.any(BigInt),
      parsers_transaction_parser_stop_prime: expect.any(BigInt),
      fixers: expect.any(BigInt),
      checkers: expect.any(BigInt),
    });

    expect(actual.transactions).toHaveLength(
      expected.statement_data.transactions.length,
    );
  });
});

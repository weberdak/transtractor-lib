use serde::{Deserialize, Serialize};
use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

#[cfg(not(target_arch = "wasm32"))]
type TimerMark = Instant;

#[cfg(target_arch = "wasm32")]
type TimerMark = f64;

#[cfg(not(target_arch = "wasm32"))]
fn timer_mark_now() -> TimerMark {
    Instant::now()
}

#[cfg(target_arch = "wasm32")]
fn timer_mark_now() -> TimerMark {
    js_performance_now()
}

#[cfg(not(target_arch = "wasm32"))]
fn timer_mark_elapsed(mark: TimerMark) -> Duration {
    mark.elapsed()
}

#[cfg(target_arch = "wasm32")]
fn timer_mark_elapsed(mark: TimerMark) -> Duration {
    Duration::from_secs_f64(((js_performance_now() - mark).max(0.0)) / 1000.0)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = performance, js_name = now)]
    fn js_performance_now() -> f64;
}

#[derive(Clone, Debug)]
pub struct Timer {
    /// Time at last start
    mark: TimerMark,
    /// Total accumulated duration of the timer
    total: Duration,
    is_running: bool,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            mark: timer_mark_now(),
            total: Duration::new(0, 0),
            is_running: false,
        }
    }

    /// Mark the timer (start or resume)
    pub fn start(&mut self) {
        if !self.is_running {
            self.mark = timer_mark_now();
            self.is_running = true;
        }
    }

    /// Reset timer and accumulate the elapsed duration
    pub fn pause(&mut self) {
        if self.is_running {
            self.total += timer_mark_elapsed(self.mark);
            self.is_running = false;
        }
    }

    /// Get the total elapsed duration of the timer in microseconds
    pub fn elapsed_micros(&self) -> u128 {
        let duration = if self.is_running {
            self.total + timer_mark_elapsed(self.mark)
        } else {
            self.total
        };
        duration.as_micros()
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct Benchmark {
    /// Time for the entire benchmarking process
    pub total: Timer,
    /// Time for extracting PDF into ordered TextItems
    pub pdf_extractor: Timer,
    /// Time for splitting TextItems by whitespace
    pub tokeniser: Timer,
    /// Time for determining statement type from tokenised TextItems
    pub typer: Timer,
    /// Time for parsing tokenised TextItems into AccountData
    pub parsers: Timer,
    /// Time for priming AccountNumberParser
    pub parsers_account_number_parser_prime: Timer,
    /// Time for parsing value AccountNumberParser
    pub parsers_account_number_parser_parse: Timer,
    /// Time for priming StartDateParser
    pub parsers_start_date_parser_prime: Timer,
    /// Time for parsing value StartDateParser
    pub parsers_start_date_parser_parse: Timer,
    /// Time for priming OpeningBalanceParser
    pub parsers_opening_balance_parser_prime: Timer,
    /// Time for parsing value OpeningBalanceParser
    pub parsers_opening_balance_parser_parse: Timer,
    /// Time for priming ClosingBalanceParser
    pub parsers_closing_balance_parser_prime: Timer,
    /// Time for parsing value ClosingBalanceParser
    pub parsers_closing_balance_parser_parse: Timer,
    /// Time for priming TransactionParser
    pub parsers_transaction_parser_start_prime: Timer,
    /// Time for parsing value TransactionParser
    pub parsers_transaction_parser_parse: Timer,
    /// Time for terminating TransactionParser
    pub parsers_transaction_parser_stop_prime: Timer,

    /// Time for filling and repairing AccountData
    pub fixers: Timer,
    /// Time for checking the final AccountData for internal consistency
    pub checkers: Timer,
}

impl Benchmark {
    pub fn new() -> Self {
        Benchmark {
            total: Timer::new(),
            pdf_extractor: Timer::new(),
            tokeniser: Timer::new(),
            typer: Timer::new(),
            parsers: Timer::new(),
            parsers_account_number_parser_prime: Timer::new(),
            parsers_account_number_parser_parse: Timer::new(),
            parsers_start_date_parser_prime: Timer::new(),
            parsers_start_date_parser_parse: Timer::new(),
            parsers_opening_balance_parser_prime: Timer::new(),
            parsers_opening_balance_parser_parse: Timer::new(),
            parsers_closing_balance_parser_prime: Timer::new(),
            parsers_closing_balance_parser_parse: Timer::new(),
            parsers_transaction_parser_start_prime: Timer::new(),
            parsers_transaction_parser_parse: Timer::new(),
            parsers_transaction_parser_stop_prime: Timer::new(),
            fixers: Timer::new(),
            checkers: Timer::new(),
        }
    }

    pub fn start_total(&mut self) {
        self.total.start();
    }
}

impl Default for Benchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark {
    pub fn as_micros(&self) -> BenchmarkMicros {
        BenchmarkMicros {
            total: self.total.elapsed_micros(),
            pdf_extractor: self.pdf_extractor.elapsed_micros(),
            tokeniser: self.tokeniser.elapsed_micros(),
            typer: self.typer.elapsed_micros(),
            parsers: self.parsers.elapsed_micros(),
            parsers_account_number_parser_prime: self
                .parsers_account_number_parser_prime
                .elapsed_micros(),
            parsers_account_number_parser_parse: self
                .parsers_account_number_parser_parse
                .elapsed_micros(),
            parsers_start_date_parser_prime: self.parsers_start_date_parser_prime.elapsed_micros(),
            parsers_start_date_parser_parse: self.parsers_start_date_parser_parse.elapsed_micros(),
            parsers_opening_balance_parser_prime: self
                .parsers_opening_balance_parser_prime
                .elapsed_micros(),
            parsers_opening_balance_parser_parse: self
                .parsers_opening_balance_parser_parse
                .elapsed_micros(),
            parsers_closing_balance_parser_prime: self
                .parsers_closing_balance_parser_prime
                .elapsed_micros(),
            parsers_closing_balance_parser_parse: self
                .parsers_closing_balance_parser_parse
                .elapsed_micros(),
            parsers_transaction_parser_start_prime: self
                .parsers_transaction_parser_start_prime
                .elapsed_micros(),
            parsers_transaction_parser_parse: self
                .parsers_transaction_parser_parse
                .elapsed_micros(),
            parsers_transaction_parser_stop_prime: self
                .parsers_transaction_parser_stop_prime
                .elapsed_micros(),
            fixers: self.fixers.elapsed_micros(),
            checkers: self.checkers.elapsed_micros(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct BenchmarkMicros {
    pub total: u128,
    pub pdf_extractor: u128,
    pub tokeniser: u128,
    pub typer: u128,
    pub parsers: u128,
    pub parsers_account_number_parser_prime: u128,
    pub parsers_account_number_parser_parse: u128,
    pub parsers_start_date_parser_prime: u128,
    pub parsers_start_date_parser_parse: u128,
    pub parsers_opening_balance_parser_prime: u128,
    pub parsers_opening_balance_parser_parse: u128,
    pub parsers_closing_balance_parser_prime: u128,
    pub parsers_closing_balance_parser_parse: u128,
    pub parsers_transaction_parser_start_prime: u128,
    pub parsers_transaction_parser_parse: u128,
    pub parsers_transaction_parser_stop_prime: u128,
    pub fixers: u128,
    pub checkers: u128,
}

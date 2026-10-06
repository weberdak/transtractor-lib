use crate::structs::AccountData;

pub mod amounts;
pub mod closing_balance;
pub mod date;
pub mod implicit_balance;
pub mod implicit_date;
pub mod opening_balance;
pub mod set_indices;
pub mod transaction_order;

pub use amounts::fix_amounts;
pub use closing_balance::fix_closing_balance;
pub use date::fix_year_crossovers;
pub use implicit_balance::fix_implicit_balances;
pub use implicit_date::fix_implicit_dates;
pub use opening_balance::fix_opening_balance;
pub use set_indices::fix_set_indices;
pub use transaction_order::fix_transaction_order;

/// Apply all fixers to the AccountData in a logical order
pub fn fix_statement_data(sd: &mut AccountData) {
    fix_implicit_dates(sd);
    fix_year_crossovers(sd);
    fix_transaction_order(sd);
    fix_opening_balance(sd);
    fix_amounts(sd);
    fix_implicit_balances(sd);
    fix_set_indices(sd);
    fix_closing_balance(sd);
}

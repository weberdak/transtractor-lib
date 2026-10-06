use crate::structs::AccountData;

pub mod balances;
pub mod fields;

pub use balances::check_balances;
pub use fields::check_fields;

/// Apply all checkers to the AccountData
pub fn check_statement_data(statement: &mut AccountData) {
    check_fields(statement);
    check_balances(statement);
}

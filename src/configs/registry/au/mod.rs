use crate::structs::StatementConfig;

pub mod cba_credit_card_1;
pub mod cba_debit_1;
pub mod cba_loan_1;
pub mod def_credit_card_1;
pub mod def_isaver_intro_1;
pub mod def_max_esaver_1;
pub mod ing_debit_1;
pub mod nab_classic_banking_1;
pub mod wbc_debit_1;

pub fn get_all_configs() -> Vec<StatementConfig> {
    let configs = vec![
        cba_credit_card_1::get_config(),
        cba_debit_1::get_config(),
        cba_loan_1::get_config(),
        def_credit_card_1::get_config(),
        def_isaver_intro_1::get_config(),
        def_max_esaver_1::get_config(),
        ing_debit_1::get_config(),
        nab_classic_banking_1::get_config(),
        wbc_debit_1::get_config(),
    ];
    configs
}

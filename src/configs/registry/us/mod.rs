use crate::structs::StatementConfig;

pub mod axp_platinum_1;
pub mod cof_360_checking_1;
pub mod cof_360_performance_savings_1;

pub fn get_all_configs() -> Vec<StatementConfig> {
    let configs = vec![
        axp_platinum_1::get_config(),
        cof_360_checking_1::get_config(),
        cof_360_performance_savings_1::get_config(),
    ];
    configs
}

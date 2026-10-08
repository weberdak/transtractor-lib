use crate::structs::StatementConfig;

pub mod bbl_savings_1;

pub fn get_all_configs() -> Vec<StatementConfig> {
    let configs = vec![bbl_savings_1::get_config()];
    configs
}

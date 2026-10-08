use crate::structs::StatementConfig;
use std::collections::HashMap;
pub mod au;
pub mod th;
pub mod us;

type RegionConfigFactory = fn() -> Vec<StatementConfig>;

pub fn get_config_map() -> HashMap<String, StatementConfig> {
    // Register all region config factories here
    let regions: Vec<RegionConfigFactory> = vec![
        au::get_all_configs,
        th::get_all_configs,
        us::get_all_configs,
    ];

    let mut config_map = HashMap::new();
    for region_builder in regions {
        for cfg in region_builder() {
            config_map.insert(cfg.key.clone(), cfg);
        }
    }

    config_map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configs::validate::validate_config;

    #[test]
    fn test_get_config_map() {
        let config_map = get_config_map();
        assert!(!config_map.is_empty());
    }

    #[test]
    fn test_validate_all_configs() {
        let config_map = get_config_map();
        for (key, cfg) in config_map {
            match validate_config(&cfg) {
                Ok(_) => (),
                Err(e) => panic!("Validation failed for config {}: {}", key, e),
            }
        }
    }
}

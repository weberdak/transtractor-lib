use crate::configs::validate::validate_config;
use crate::structs::statement_config::StatementConfig;
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::path::Path;

fn compile_regex_vec(patterns: Vec<String>) -> Result<Vec<Regex>, String> {
    let mut result = Vec::with_capacity(patterns.len());
    for p in patterns {
        match Regex::new(&p) {
            Ok(r) => result.push(r),
            Err(e) => return Err(format!("Invalid regex '{}': {}", p, e)),
        }
    }
    Ok(result)
}

/// Result type containing both configuration and any deprecated fields found
#[derive(Debug)]
pub struct ConfigParseResult {
    pub config: StatementConfig,
    pub deprecated_fields: Vec<String>,
}

/// Raw struct used only for deserialization (all fields optional so we can overlay defaults)
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StatementConfigPartial {
    key: Option<String>,
    bank_name: Option<String>,
    account_type: Option<String>,
    account_terms: Option<Vec<String>>,
    account_examples: Option<Vec<String>>,

    account_number_terms: Option<Vec<String>>,
    account_number_trigger_count: Option<usize>,
    account_number_patterns: Option<Vec<String>>,
    account_number_alignment: Option<String>,
    account_number_alignment_tol: Option<i32>,

    opening_balance_terms: Option<Vec<String>>,
    opening_balance_trigger_count: Option<usize>,
    opening_balance_formats: Option<Vec<String>>,
    opening_balance_alignment: Option<String>,
    opening_balance_alignment_tol: Option<i32>,
    opening_balance_invert: Option<bool>,

    closing_balance_terms: Option<Vec<String>>,
    closing_balance_trigger_count: Option<usize>,
    closing_balance_formats: Option<Vec<String>>,
    closing_balance_alignment: Option<String>,
    closing_balance_alignment_tol: Option<i32>,
    closing_balance_invert: Option<bool>,

    start_date_terms: Option<Vec<String>>,
    start_date_trigger_count: Option<usize>,
    start_date_formats: Option<Vec<String>>,
    start_date_alignment: Option<String>,
    start_date_alignment_tol: Option<i32>,

    transaction_terms: Option<Vec<String>>,
    transaction_trigger_count: Option<usize>,
    transaction_terms_stop: Option<Vec<String>>,
    transaction_stop_trigger_count: Option<usize>,
    transaction_formats: Option<Vec<Vec<String>>>,
    transaction_start_date_required: Option<bool>,
    transaction_alignment_tol: Option<i32>,

    transaction_date_formats: Option<Vec<String>>,
    transaction_date_headers: Option<Vec<String>>,
    transaction_date_alignment: Option<String>,
    transaction_date_x1_range: Option<[i32; 2]>,
    transaction_date_x2_range: Option<[i32; 2]>,

    transaction_description_headers: Option<Vec<String>>,
    transaction_description_alignment: Option<String>,
    transaction_description_x1_range: Option<[i32; 2]>,
    transaction_description_x2_range: Option<[i32; 2]>,
    transaction_description_exclude: Option<Vec<String>>,

    transaction_amount_formats: Option<Vec<String>>,
    transaction_amount_headers: Option<Vec<String>>,
    transaction_amount_alignment: Option<String>,
    transaction_amount_x1_range: Option<[i32; 2]>,
    transaction_amount_x2_range: Option<[i32; 2]>,
    transaction_amount_invert_headers: Option<Vec<String>>,
    transaction_amount_invert_alignment: Option<String>,
    transaction_amount_invert_x1_range: Option<[i32; 2]>,
    transaction_amount_invert_x2_range: Option<[i32; 2]>,
    transaction_amount_invert: Option<bool>,

    transaction_balance_formats: Option<Vec<String>>,
    transaction_balance_headers: Option<Vec<String>>,
    transaction_balance_alignment: Option<String>,
    transaction_balance_x1_range: Option<[i32; 2]>,
    transaction_balance_x2_range: Option<[i32; 2]>,
    transaction_balance_invert: Option<bool>,
    transaction_balance_ignore: Option<bool>,

    // Deprecated fields (kept for v0.10.0 compatibility)
    fix_text_order: Option<serde_json::Value>,
    transaction_new_line_tol: Option<serde_json::Value>,
    start_date_first_match: Option<serde_json::Value>,
}

pub fn from_json_file<P: AsRef<Path>>(path: P) -> Result<StatementConfig, String> {
    let path_ref = path.as_ref();
    let data = fs::read_to_string(&path)
        .map_err(|e| format!("Failed reading config {:?}: {}", path_ref, e))?;
    let cfg = from_json_str(&data)?;
    Ok(cfg)
}

/// Internal function that returns both config and deprecated fields
pub fn from_json_str_with_deprecations(src: &str) -> Result<ConfigParseResult, String> {
    let partial: StatementConfigPartial =
        serde_json::from_str(src).map_err(|e| format!("JSON parse error: {}", e))?;
    let mut cfg = StatementConfig::default();
    let mut deprecated_fields = Vec::new();

    macro_rules! overlay {
        ($field:ident) => {
            if let Some(v) = partial.$field {
                cfg.$field = v;
            }
        };
    }

    // Check for deprecated fields
    if partial.fix_text_order.is_some() {
        deprecated_fields.push("fix_text_order (deprecated since v0.10.0)".to_string());
    }
    if partial.transaction_new_line_tol.is_some() {
        deprecated_fields.push("transaction_new_line_tol (deprecated since v0.10.0)".to_string());
    }
    if partial.start_date_first_match.is_some() {
        deprecated_fields.push("start_date_first_match (deprecated since v0.13.0)".to_string());
    }

    overlay!(key);
    overlay!(bank_name);
    overlay!(account_type);
    overlay!(account_terms);
    overlay!(account_examples);

    overlay!(account_number_terms);
    overlay!(account_number_trigger_count);
    if let Some(patterns) = partial.account_number_patterns {
        cfg.account_number_patterns = compile_regex_vec(patterns)?;
    }
    overlay!(account_number_alignment);
    overlay!(account_number_alignment_tol);

    overlay!(opening_balance_terms);
    overlay!(opening_balance_trigger_count);
    overlay!(opening_balance_formats);
    overlay!(opening_balance_alignment);
    overlay!(opening_balance_alignment_tol);
    overlay!(opening_balance_invert);

    overlay!(closing_balance_terms);
    overlay!(closing_balance_trigger_count);
    overlay!(closing_balance_formats);
    overlay!(closing_balance_alignment);
    overlay!(closing_balance_alignment_tol);
    overlay!(closing_balance_invert);

    overlay!(start_date_terms);
    overlay!(start_date_trigger_count);
    overlay!(start_date_formats);
    overlay!(start_date_alignment);
    overlay!(start_date_alignment_tol);

    overlay!(transaction_terms);
    overlay!(transaction_trigger_count);
    overlay!(transaction_terms_stop);
    overlay!(transaction_stop_trigger_count);
    overlay!(transaction_formats);
    overlay!(transaction_start_date_required);
    overlay!(transaction_alignment_tol);

    overlay!(transaction_date_formats);
    overlay!(transaction_date_headers);
    overlay!(transaction_date_alignment);
    overlay!(transaction_date_x1_range);
    overlay!(transaction_date_x2_range);

    overlay!(transaction_description_headers);
    overlay!(transaction_description_alignment);
    overlay!(transaction_description_x1_range);
    overlay!(transaction_description_x2_range);

    if let Some(ex_patterns) = partial.transaction_description_exclude {
        cfg.transaction_description_exclude = compile_regex_vec(ex_patterns)?;
    }

    overlay!(transaction_amount_formats);
    overlay!(transaction_amount_headers);
    overlay!(transaction_amount_alignment);
    overlay!(transaction_amount_x1_range);
    overlay!(transaction_amount_x2_range);
    overlay!(transaction_amount_invert_headers);
    overlay!(transaction_amount_invert_alignment);
    overlay!(transaction_amount_invert_x1_range);
    overlay!(transaction_amount_invert_x2_range);
    overlay!(transaction_amount_invert);

    overlay!(transaction_balance_formats);
    overlay!(transaction_balance_headers);
    overlay!(transaction_balance_alignment);
    overlay!(transaction_balance_x1_range);
    overlay!(transaction_balance_x2_range);
    overlay!(transaction_balance_invert);
    overlay!(transaction_balance_ignore);

    validate_config(&cfg).map_err(|e| format!("Config validation error: {}", e))?;
    Ok(ConfigParseResult {
        config: cfg,
        deprecated_fields,
    })
}

pub fn from_json_str(src: &str) -> Result<StatementConfig, String> {
    let result = from_json_str_with_deprecations(src)?;
    Ok(result.config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transaction_x_ranges_are_loaded_and_defaulted() {
        let mut config_json: serde_json::Value =
            serde_json::from_str(include_str!("../../../tests/fixtures/test1_config.json"))
                .unwrap();
        let fields = [
            ("transaction_date_x1_range", serde_json::json!([1, 2])),
            ("transaction_date_x2_range", serde_json::json!([3, 4])),
            (
                "transaction_description_x1_range",
                serde_json::json!([5, 6]),
            ),
            (
                "transaction_description_x2_range",
                serde_json::json!([7, 8]),
            ),
            ("transaction_amount_x1_range", serde_json::json!([9, 10])),
            ("transaction_amount_x2_range", serde_json::json!([11, 12])),
            (
                "transaction_amount_invert_x1_range",
                serde_json::json!([13, 14]),
            ),
            (
                "transaction_amount_invert_x2_range",
                serde_json::json!([15, 16]),
            ),
            ("transaction_balance_x1_range", serde_json::json!([17, 18])),
            ("transaction_balance_x2_range", serde_json::json!([19, 20])),
        ];
        let object = config_json.as_object_mut().unwrap();
        for (key, value) in fields {
            object.insert(key.to_string(), value);
        }
        let config = from_json_str(&config_json.to_string()).unwrap();

        assert_eq!(config.transaction_date_x1_range, [1, 2]);
        assert_eq!(config.transaction_date_x2_range, [3, 4]);
        assert_eq!(config.transaction_description_x1_range, [5, 6]);
        assert_eq!(config.transaction_description_x2_range, [7, 8]);
        assert_eq!(config.transaction_amount_x1_range, [9, 10]);
        assert_eq!(config.transaction_amount_x2_range, [11, 12]);
        assert_eq!(config.transaction_amount_invert_x1_range, [13, 14]);
        assert_eq!(config.transaction_amount_invert_x2_range, [15, 16]);
        assert_eq!(config.transaction_balance_x1_range, [17, 18]);
        assert_eq!(config.transaction_balance_x2_range, [19, 20]);

        let default_config =
            from_json_str(include_str!("../../../tests/fixtures/test1_config.json")).unwrap();
        assert_eq!(default_config.transaction_date_x1_range, [0, 10000]);
        assert_eq!(default_config.transaction_date_x2_range, [0, 10000]);
        assert_eq!(default_config.transaction_description_x1_range, [0, 10000]);
        assert_eq!(default_config.transaction_description_x2_range, [0, 10000]);
        assert_eq!(default_config.transaction_amount_x1_range, [0, 10000]);
        assert_eq!(default_config.transaction_amount_x2_range, [0, 10000]);
        assert_eq!(
            default_config.transaction_amount_invert_x1_range,
            [0, 10000]
        );
        assert_eq!(
            default_config.transaction_amount_invert_x2_range,
            [0, 10000]
        );
        assert_eq!(default_config.transaction_balance_x1_range, [0, 10000]);
        assert_eq!(default_config.transaction_balance_x2_range, [0, 10000]);
    }
}

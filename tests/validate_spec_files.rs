use std::fs;
use std::path::Path;
use transtractor::configs::db::ConfigDB;
use transtractor::configs::registry::get_config_map;
use transtractor::structs::Spec;

fn validate_spec_file_name(spec_path: &Path) -> Result<(), String> {
    let file_name = spec_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("{}: missing or invalid file name", spec_path.display()))?;

    if file_name != file_name.to_lowercase() {
        return Err(format!(
            "{}: file name {:?} must be lowercase",
            spec_path.display(),
            file_name
        ));
    }

    let dir = spec_path
        .parent()
        .and_then(|parent| parent.file_name())
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("{}: missing parent directory name", spec_path.display()))?;
    let file_stem = spec_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| format!("{}: missing or invalid file stem", spec_path.display()))?;

    let components: Vec<&str> = file_stem.split("__").collect();
    if components.len() != 3 {
        return Err(format!(
            "{}: expected exactly 3 filename components separated by double underscores, found {} in {:?}",
            spec_path.display(),
            components.len(),
            components
        ));
    }

    let prefix = format!("{}__{}__", dir, components[0]);
    if !get_config_map().keys().any(|key| key.starts_with(&prefix)) {
        return Err(format!(
            "{}: no registered config key starts with {:?} (directory + first filename component)",
            spec_path.display(),
            prefix
        ));
    }

    if components[2].parse::<i64>().is_err() {
        return Err(format!(
            "{}: third filename component {:?} is not a valid integer",
            spec_path.display(),
            components[2]
        ));
    }

    Ok(())
}

fn validate_spec_file(spec_path: &str) {
    let spec_path = Path::new(spec_path);
    let config_db = ConfigDB::new();
    let spec_content = fs::read_to_string(spec_path)
        .unwrap_or_else(|error| panic!("{}: failed to read file: {error}", spec_path.display()));
    let spec = Spec::from_json(&spec_content).unwrap_or_else(|error| {
        panic!(
            "{}: failed to parse JSON spec: {error}",
            spec_path.display()
        )
    });
    validate_spec_file_name(spec_path).unwrap_or_else(|error| panic!("{error}"));
    spec.validate(&config_db)
        .unwrap_or_else(|error| panic!("{}:\n{error}", spec_path.display()));
}

include!(concat!(env!("OUT_DIR"), "/spec_tests.rs"));

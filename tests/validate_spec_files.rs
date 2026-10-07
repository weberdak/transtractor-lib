use std::fs;
use std::path::{Path, PathBuf};
use transtractor::configs::db::ConfigDB;
use transtractor::configs::registry::get_config_map;
use transtractor::structs::Spec;

fn collect_json_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("Failed to read directory {}: {}", dir.display(), e));

    for entry in entries {
        let entry = entry.unwrap_or_else(|e| {
            panic!(
                "Failed to read an entry in directory {}: {}",
                dir.display(),
                e
            )
        });
        let path = entry.path();

        if path.is_dir() {
            collect_json_files(&path, out);
        } else if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
        {
            out.push(path);
        }
    }
}

fn collect_spec_files() -> Vec<PathBuf> {
    let fixtures_root = Path::new("tests/fixtures/spec");
    assert!(
        fixtures_root.exists(),
        "Spec fixtures directory does not exist: {}",
        fixtures_root.display()
    );

    let mut spec_files = Vec::new();
    collect_json_files(fixtures_root, &mut spec_files);
    spec_files.sort();

    assert!(
        !spec_files.is_empty(),
        "No JSON spec fixtures found under {}",
        fixtures_root.display()
    );

    spec_files
}

/// Config key implied by a spec path: `<dir>/<a>__<b>__<c>__<name>__<n>.json` -> `<dir>__<a>__<b>__<c>`.
fn spec_key_from_path(spec_path: &Path) -> Result<String, String> {
    let dir = spec_path
        .parent()
        .and_then(|parent| parent.file_name())
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("{}: missing parent directory name", spec_path.display()))?;
    let stem = spec_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| format!("{}: missing or invalid file stem", spec_path.display()))?;
    let components: Vec<&str> = stem.split("__").collect();
    if components.len() < 3 {
        return Err(format!(
            "{}: expected at least 3 filename components",
            spec_path.display()
        ));
    }
    Ok(format!("{}__{}", dir, components[..3].join("__")))
}

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

    let file_stem = spec_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| format!("{}: missing or invalid file stem", spec_path.display()))?;

    let components: Vec<&str> = file_stem.split("__").collect();
    if components.len() != 5 {
        return Err(format!(
            "{}: expected exactly 5 filename components separated by double underscores, found {} in {:?}",
            spec_path.display(),
            components.len(),
            components
        ));
    }

    let key = spec_key_from_path(spec_path)?;
    if !get_config_map().contains_key(&key) {
        return Err(format!(
            "{}: derived config key {:?} (directory + first three filename components) is not a registered config",
            spec_path.display(),
            key
        ));
    }

    if components[4].parse::<i64>().is_err() {
        return Err(format!(
            "{}: fifth filename component {:?} is not a valid integer",
            spec_path.display(),
            components[4]
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

#[test]
fn every_registered_config_has_a_spec_file() {
    let spec_files = collect_spec_files();
    let mut spec_keys = std::collections::HashSet::new();
    let mut read_failures = Vec::new();

    for spec_path in spec_files {
        match spec_key_from_path(&spec_path) {
            Ok(key) => {
                spec_keys.insert(key);
            }
            Err(error) => read_failures.push(error),
        }
    }

    assert!(
        read_failures.is_empty(),
        "{} spec path failure(s):\n\n{}",
        read_failures.len(),
        read_failures.join("\n\n")
    );

    let config_map = get_config_map();
    let mut missing = config_map
        .keys()
        .filter(|key| !spec_keys.contains(*key))
        .cloned()
        .collect::<Vec<String>>();
    missing.sort();

    assert!(
        missing.is_empty(),
        "{} config(s) are missing associated spec files in tests/fixtures/spec:\n{}",
        missing.len(),
        missing.join("\n")
    );
}

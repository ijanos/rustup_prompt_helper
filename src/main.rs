extern crate toml;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;

fn strip_toolchain(name: &str) -> String {
    name.split('-').nth(0).unwrap().to_string()
}

fn read_settings() -> (String, BTreeMap<String, String>) {
    let mut settings_path = env::home_dir().unwrap();
    settings_path.push(".multirust/settings.toml");

    let mut file = File::open(&settings_path).expect("file open error");

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("read error");

    let value = toml::Parser::new(&buffer).parse().unwrap();
    let default = value.get("default_toolchain").unwrap().as_str().unwrap();
    let overrides = value.get("overrides").unwrap().as_table().unwrap();
    let overrides = overrides.iter()
        .map(|(k, v)| (k.to_string(), v.as_str().unwrap().to_string()))
        .collect();

    (default.into(), overrides)
}

fn main() {
    let cwd = env::current_dir().unwrap();
    let (default, overrides) = read_settings();

    let toolchain = match overrides.get(cwd.to_str().unwrap()) {
        Some(toolchain) => strip_toolchain(toolchain),
        None => default,
    };

    println!("{}", toolchain);
}

extern crate toml;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;

fn strip_toolchain(name: &toml::Value) -> String {
    name.as_str().unwrap().split('-').nth(0).unwrap().to_string()
}

fn read_settings() -> (String, BTreeMap<String, toml::Value>) {
    let mut settings_path = env::home_dir().expect("Cannot get home directory");
    settings_path.push(".multirust/settings.toml");

    let mut file = File::open(&settings_path).expect("Error opening settings.toml file");

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Error reading settings.toml");

    let value = toml::Parser::new(&buffer).parse().expect("Error parsing settings.toml");
    let default = value.get("default_toolchain")
        .expect("Did not found default_toolchain in settings.toml")
        .as_str()
        .unwrap();
    let overrides = value.get("overrides")
        .expect("Did not found overrides in settings.toml")
        .as_table()
        .unwrap();

    (default.to_owned(), overrides.to_owned())
}

fn main() {
    let cwd = env::current_dir().expect("Cannot get working directory");
    let cwd = cwd.to_str().expect("Cannot convert working directory to string");
    let (default, overrides) = read_settings();

    let toolchain = if let Some(toolchain) = overrides.get(cwd) {
        strip_toolchain(toolchain)
    } else {
        default
    };

    println!("{}", toolchain);
}

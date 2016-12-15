extern crate toml;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;

fn strip_toolchain(name: &toml::Value) -> Option<&str> {
    name.as_str().and_then(|n| n.split('-').nth(0))
}

fn read_settings() -> Result<(String, BTreeMap<String, toml::Value>), String> {
    let mut settings_path = try!(env::home_dir().ok_or("cannot get home directory"));
    settings_path.push(".rustup/settings.toml");

    let mut file = try!(File::open(&settings_path).map_err(|_| "cannot open settings.toml file"));

    let mut buffer = String::new();
    try!(file.read_to_string(&mut buffer).map_err(|_| "cannot read settings.toml"));

    let value =
        try!(toml::Parser::new(&buffer).parse().ok_or("cannot parse settings.toml as toml"));
    let default = try!(value.get("default_toolchain")
        .and_then(|d| d.as_str())
        .ok_or("did not found default_toolchain in settings.toml"));
    let overrides = try!(value.get("overrides")
        .and_then(|t| t.as_table())
        .ok_or("did not found overrides in settings.toml"));

    Ok((default.to_owned(), overrides.to_owned()))
}

fn get_toolchain() -> Result<String, String> {
    let cwd = try!(env::current_dir().map_err(|_| "cannot get working directory"));
    let cwd = try!(cwd.to_str().ok_or("cannot convert working directory to string"));
    let (default, overrides) = try!(read_settings());

    match overrides.get(cwd) {
        Some(toolchain) => {
            let toolchain = try!(strip_toolchain(toolchain).ok_or("Can not parse settings.toml"));
            Ok(toolchain.to_owned())
        }
        None => Ok(default),
    }
}

fn main() {
    match get_toolchain() {
        Ok(toolchain) => println!("{}", toolchain),
        Err(message) => {
            if env::args().len() > 1 {
                println!("Error: {}", message);
            }
        }
    }
}

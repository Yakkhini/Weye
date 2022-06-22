use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::prelude::*;

/*
enum AvailableNameSlice {
    User(String),
    Year(String),
    Month(String),
    Minute(String),
    Second(String),
}
*/

#[derive(Serialize, Deserialize, Debug)]
struct SingleConfig {
    delay: i32,
    save_path: String,
    file_name: String,
    save_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    config_selecte: SingleConfig,
    config_window: SingleConfig,
    config_fulshot: SingleConfig,
}

pub fn config_parse() -> Config {
    let config_path = "/home/vortexlove/.config/weye/config.toml";
    let mut config_file = match File::open(config_path) {
        Ok(file) => file,
        Err(e) => panic!("no such config file {} exception:{}", config_path, e),
    };
    let mut config_text = String::new();
    match config_file.read_to_string(&mut config_text) {
        Ok(string) => string,
        Err(e) => panic!("Error Reading file: {}", e),
    };

    let config: Config = toml::from_str(&config_text).unwrap();

    return config;
}

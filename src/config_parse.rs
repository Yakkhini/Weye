use serde::{Deserialize, Serialize};

use chrono::prelude::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

struct AvailableNameSlice {
    home: String,
    user: String,
    year: String,
    month: String,
    day: String,
    hour: String,
    minute: String,
    second: String,
    nanosecond: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleConfig {
    delay: i32,
    save_path: String,
    file_name: String,
    save_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub config_selecte: SingleConfig,
    pub config_window: SingleConfig,
    pub config_fulshot: SingleConfig,
}

pub fn config_parse() -> Config {
    let home = match env::var_os("HOME") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("Can't find HOME env."),
    };
    let config_path = home + "/.config/weye/config.toml";
    let mut config_file = match File::open(&config_path) {
        Ok(file) => file,
        Err(e) => panic!("no such config file {} exception:{}", &config_path, e),
    };
    let mut config_text = String::new();
    match config_file.read_to_string(&mut config_text) {
        Ok(string) => string,
        Err(e) => panic!("Error Reading file: {}", e),
    };

    let config: Config = toml::from_str(&config_text).unwrap();

    return config;
}

fn name_slice() -> AvailableNameSlice {
    let user = match env::var_os("USER") {
        Some(v) => v.into_string().unwrap(),
        None => "notset".to_string(),
    };

    let home = match env::var_os("HOME") {
        Some(v) => v.into_string().unwrap(),
        None => "notset".to_string(),
    };

    let year = Local::now().year().to_string();
    let month = Local::now().month().to_string();
    let day = Local::now().day().to_string();
    let hour = Local::now().hour().to_string();
    let minute = Local::now().minute().to_string();
    let second = Local::now().second().to_string();
    let nanosecond = Local::now().nanosecond().to_string();

    let avai_slice = AvailableNameSlice {
        home,
        user,
        year,
        month,
        day,
        hour,
        minute,
        second,
        nanosecond,
    };

    return avai_slice;
}

/// generate image save path for grimshot.
pub fn save_path_gen(single_config: &SingleConfig) -> String {
    let mut path = String::new();

    path += single_config.save_path.as_str();
    path += save_name_gen(&single_config.file_name).as_str();

    return path;
}

fn save_name_gen(raw_name: &String) -> String {
    let mut name = String::new();

    let name_vec: Vec<&str> = raw_name.trim().split(" + ").collect();

    let avai_slice = name_slice();

    for slice in name_vec {
        match slice {
            "user" => {
                name += &avai_slice.user;
            }
            "home" => {
                name += &avai_slice.home;
            }
            "year" => {
                name += &avai_slice.year;
            }
            "month" => {
                name += &avai_slice.month;
            }
            "day" => {
                name += &avai_slice.day;
            }
            "hour" => {
                name += &avai_slice.hour;
            }
            "minute" => {
                name += &avai_slice.minute;
            }
            "second" => {
                name += &avai_slice.second;
            }
            "nanosecond" => {
                name += &avai_slice.nanosecond;
            }
            _ => name += slice,
        }
    }

    return name;
}

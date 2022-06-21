/*
Copyright (c) 2022 Yakkhini
Weye is licensed under Mulan PSL v2.
You can use this software according to the terms and conditions of the Mulan PSL v2.
You may obtain a copy of Mulan PSL v2 at:
         http://license.coscl.org.cn/MulanPSL2
THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
See the Mulan PSL v2 for more details.
*/

use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

use gtk::prelude::*;
use gtk::Application;

use libappindicator::{AppIndicator, AppIndicatorStatus};

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
struct Config {
    config_selecte: SingleConfig,
    config_window: SingleConfig,
    config_fulshot: SingleConfig,
}

fn main() {
    let config_path = "/home/vortexlove/.config/weye/config.toml";
    let mut config_file = match File::open(config_path) {
        Ok(f) => f,
        Err(e) => panic!("no such config file {} exception:{}", config_path, e),
    };
    let mut config_text = String::new();
    match config_file.read_to_string(&mut config_text) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e),
    };

    let _config: Config = toml::from_str(&config_text).unwrap(); // handle it in future.

    let app = Application::builder()
        .application_id("com.github.yakkhini.weye")
        .build();

    app.hold();

    app.connect_activate(build_tray);

    app.run();
}

fn build_tray(_app: &Application) {
    let mut indicator = AppIndicator::new("libappindicator test application", "");
    indicator.set_status(AppIndicatorStatus::Active);
    let icon_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
    indicator.set_icon_theme_path(icon_path.to_str().unwrap());
    indicator.set_icon_full("screenshot-one", "icon");

    let mut menu = gtk::Menu::new();
    let menu_fulshot = gtk::MenuItem::with_label("Full-screen shot");
    let menu_exit = gtk::MenuItem::with_label("Exit");
    menu_fulshot.connect_activate(|_| {
        let save_path = Path::new("$XDG_SCREENSHOTS_DIR/$USER@$HOST_`date +\"%Y%m%d%H%M%S\".png`");
        Command::new("grimshot")
            .arg("save")
            .arg("output")
            .arg(save_path)
            .output()
            .expect("grimshot excute failed");
    });
    menu_exit.connect_activate(|_| {
        gtk::main_quit();
    });
    menu.append(&menu_fulshot);
    menu.append(&menu_exit);
    indicator.set_menu(&mut menu);
    menu.show_all();
}

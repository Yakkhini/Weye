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

use gtk::prelude::*;

use libappindicator::{AppIndicator, AppIndicatorStatus};
use std::path::Path;
use std::process::Command;
use std::{thread, time};

use crate::config_parse;

pub fn build_tray() {
    let mut indicator = AppIndicator::new("libappindicator test application", "");
    indicator.set_status(AppIndicatorStatus::Active);

    let icon_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
    indicator.set_icon_theme_path(icon_path.to_str().unwrap());
    indicator.set_icon_full("screenshot-one", "icon");

    let mut menu = gtk::Menu::new();

    let menu_select = gtk::MenuItem::with_label("Screenshot");
    let menu_window = gtk::MenuItem::with_label("Window shot");
    let menu_fulshot = gtk::MenuItem::with_label("Full-screen shot");
    let menu_exit = gtk::MenuItem::with_label("Exit");

    menu_select.connect_activate(|_| {
        let config = config_parse::config_parse();

        let select_path = config_parse::save_path_gen(&config.config_selecte);

        let delay = time::Duration::from_secs(config.config_selecte.delay.try_into().unwrap());

        screenshot_command(&select_path, &delay, "area");
    });

    menu_window.connect_activate(|_| {
        let config = config_parse::config_parse();

        let window_path = config_parse::save_path_gen(&config.config_window);

        let delay = time::Duration::from_secs(config.config_window.delay.try_into().unwrap());

        screenshot_command(&window_path, &delay, "window");
    });

    menu_fulshot.connect_activate(|_| {
        let config = config_parse::config_parse();

        let fulshot_path = config_parse::save_path_gen(&config.config_fulshot);

        let delay = time::Duration::from_secs(config.config_fulshot.delay.try_into().unwrap());

        screenshot_command(&fulshot_path, &delay, "output");
    });

    menu_exit.connect_activate(|_| {
        gtk::main_quit();
    });

    menu.append(&menu_select);
    menu.append(&menu_window);
    menu.append(&menu_fulshot);
    menu.append(&menu_exit);
    indicator.set_menu(&mut menu);

    menu.show_all();
}

fn screenshot_command(path: &String, delay: &time::Duration, shot_type: &str) {
    let notify = path.clone() + " saved";
    thread::sleep(*delay);

    Command::new("grimshot")
        .arg("save")
        .arg(shot_type)
        .arg(path)
        .output()
        .expect("grimshot excute failed");

    thread::sleep(time::Duration::from_secs(2));

    Command::new("notify-send")
        .arg("--app-name=Weye")
        .arg("--expire-time=3000")
        .arg(notify)
        .output()
        .expect("notify excute failed");
}

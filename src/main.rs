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

use std::path::Path;
use std::process::Command;

use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;

use libappindicator::{AppIndicator, AppIndicatorStatus};

fn main() {
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
        let save_path = Path::new("/diske/Rust/weye/1.png");
        Command::new("grimshot")
            .arg("save")
            .arg("output")
            .arg(save_path)
            .output().expect("grimshot excute failed");
    });
    menu_exit.connect_activate(|_| {
        gtk::main_quit();
    });
    menu.append(&menu_fulshot);
    menu.append(&menu_exit);
    indicator.set_menu(&mut menu);
    menu.show_all();
}

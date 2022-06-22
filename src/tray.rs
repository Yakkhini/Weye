use gtk::prelude::*;

use libappindicator::{AppIndicator, AppIndicatorStatus};
use std::path::Path;
use std::process::Command;

use crate::config_parse::config_parse;

pub fn build_tray() {
    let mut _config = config_parse();

    let mut indicator = AppIndicator::new("libappindicator test application", "");
    indicator.set_status(AppIndicatorStatus::Active);
    let icon_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
    indicator.set_icon_theme_path(icon_path.to_str().unwrap());
    indicator.set_icon_full("screenshot-one", "icon");

    let mut menu = gtk::Menu::new();
    let menu_fulshot = gtk::MenuItem::with_label("Full-screen shot");
    let menu_exit = gtk::MenuItem::with_label("Exit");

    menu_fulshot.connect_activate(|_| {
        let save_path = Path::new("/diske/Rust/weye/1.png`");
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

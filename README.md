# Weye

A lightweight screenshot tool for sway.

## Installation

Weye is available in AUR repo, so Arch Linux user can use your favorite AUR helper to install it.

```bash
yay -S weye
```

## Usage

### How to shot

The usage of this software is simple. When it started, a icon will appear in your system tray. Click it then the tray icon would pop up a menu, which contains screenshot methods.

### How to config

Config path is default to `$HOME/.config/weye/config.toml`.

Here is default config template:

```toml
[config_selecte]
delay = 0
save_path = "/path/to/your/save/path/"
file_name = "Screenshot- + user + @ + year + month + day + hour + minute"
save_type = "png"

[config_window]
delay = 0
save_path = "/path/to/your/save/path/"
file_name = "Screenshot- + user + @ + year + month + day + hour + minute"
save_type = "png"

[config_fulshot]
delay = 0
save_path = "/path/to/your/save/path/"
file_name = "Screenshot- + user + @ + year + month + day + hour + minute"
save_type = "png"
```

Here is three rules you should know:

1. `delay` means the time after click screenshot button then Weye waiting for(seconds). It should be int type.
2. `save_path` should **end with "/" character.** And it isn't support system environment variables now.
3. `file_name` is a string including some built-in variables and custom string slice, connect with " + " slice(**Note the space arounding plus symbol**). When your slice isn't match any built-in availiable variables, it should be used directly in the output file name, like "@" and "Screenshot-" slices in template.

An output file in template:

```
/path/to/your/save/path/Screenshot-username@202206231427.png
```

Availiable variables:

- user: user name.
- year: current year.
- month: current month, range in 1-12.
- day: current day in current month, range in 0-31.
- hour: current hour in current day, range in 0-23.
- minute: current minute in current hour, range in 0-60.
- nanominute: current nanominute in current minute.

### How to exit

Click tray icon -> Exit is built-in method to end process. Otherwise you can use your process manager like htop, or just pkill to kill the process.

Just do what you want to do cause software don't pain. ;)

## License

```
Weye is licensed under Mulan PSL v2.
You can use this software according to the terms and conditions of the Mulan PSL v2.
You may obtain a copy of Mulan PSL v2 at:
         http://license.coscl.org.cn/MulanPSL2
THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
See the Mulan PSL v2 for more details.
```

Mulan PSL v2 is compatible with MIT License, so the usage of gtk-rs crate is legal.

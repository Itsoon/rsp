use once_cell::sync::Lazy;
use serde::Deserialize;
use std::fs;
use std::process::exit;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub linux: Option<bool>,
    pub hyprland: Option<bool>,
    pub debug: Option<bool>,
    pub default_profile: Option<bool>,
}

fn load_settings() -> Settings {
    let home_dir = match std::env::var("HOME") {
        Ok(val) => val,
        Err(_) => panic!("HOME environment variable is not set"),
    };

    let settings_path = format!("{}/.config/rsp/settings.toml", home_dir);

    let raw_settings = match fs::read_to_string(&settings_path) {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            exit(1);
        }
    };

    match toml::from_str(&raw_settings) {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!("TOML parsing errors: {}", e);
            exit(1);
        }
    }
}

pub static SETTINGS: Lazy<Settings> = Lazy::new(|| load_settings());

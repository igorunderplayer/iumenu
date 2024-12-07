use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub window: WindowConfig,
}

#[derive(Deserialize)]
pub struct WindowConfig {
    pub width: i32,
    pub height: i32,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            window: WindowConfig::default(),
        }
    }
}

impl Default for WindowConfig {
    fn default() -> WindowConfig {
        WindowConfig {
            width: 800,
            height: 400,
        }
    }
}

pub fn get_default_path() -> PathBuf {
    PathBuf::from_str(".config/iumenu/config.toml").unwrap()
}

pub fn load_from_file(path: &PathBuf) -> Config {
    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read config file!");
            std::process::exit(1)
        }
    };

    let data: Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load config file!");
            std::process::exit(1)
        }
    };

    return data;
}

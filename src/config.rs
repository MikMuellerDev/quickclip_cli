use std::{fs, process};
extern crate serde;
extern crate serde_json;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]

pub struct Config {
    pub quickclip_url: String,
    pub quicklip_username: String,
    pub quicklip_password: String,
    pub default_clipboard_id: String,
}

pub fn read_config(filepath: &str) -> Config {
    let config_string = fs::read_to_string(filepath).ok().unwrap_or_else(|| {
        eprintln!("Reading Config file failed.");
        process::exit(2)
    });
    let config: Config = serde_json::from_str(config_string.as_str())
        .ok()
        .unwrap_or_else(|| {
            eprintln!("Parsing config file failed, check your JSON syntax.");
            process::exit(3)
        });
    return config;
}

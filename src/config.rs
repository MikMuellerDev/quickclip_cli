extern crate serde_json;
use serde::Deserialize;
use std::{fs, process};
use serde::Serialize;
extern crate serde;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub quickclip_url: String,
    pub quicklip_username: String,
    pub quicklip_password: String,
    pub default_clipboard_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Clipboard {
    pub name: String,
    pub id: String,
    pub content: String,
    pub description: String,
    pub restricted: bool,
    pub refresh: bool,
    pub refresh_interval: i32,
    pub read_only: bool,
}

pub fn read_config(filepath: &str) -> Config {
    let config_string = fs::read_to_string(filepath).ok().unwrap_or_else(|| {
        eprintln!("Reading Config file from {} failed.", filepath);
        process::exit(2)
    });
    let config: Config = serde_json::from_str(config_string.as_str())
        .ok()
        .unwrap_or_else(|| {
            eprintln!("Parsing config file failed, check your JSON syntax.");
            process::exit(3)
        });
    config
}

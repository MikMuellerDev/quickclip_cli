use std::io;

use crate::config::Config;
use structopt::StructOpt;
mod colors;
mod config;
mod quickclip;

const CONFIG_FILE_PATH: &str = "./config/config.json";

/// A CLI to interact with QuickClip from the terminal
#[derive(StructOpt, Debug)]
#[structopt(author)]
struct QuickClip {
    /// Method of interacting with QuickClip. Valid modes are ("set", "s") and ("get", "g")
    #[structopt(required = true)]
    mode: String,

    /// If used, quickclip CLI will read the content from StdIn, which allows piping content into quickclip.
    #[structopt(short = "i", long = "input")]
    input: bool,

    /// Content to push onto the clipboard when using the set mode
    #[structopt(name = "CONTENT")]
    content: Option<String>,
}

#[tokio::main]
async fn main() {
    let config: Config = config::read_config(CONFIG_FILE_PATH);
    // println!("{:?}", &config);
    // println!("Config file loaded {}.", colors::green("successfully"));

    let quickclip = QuickClip::from_args();

    if quickclip.mode == "set" || quickclip.mode == "s" {
        let mut content: String;
        if !quickclip.input {
            content = quickclip.content.unwrap().to_string();
        } else {
            content = "".to_string();
            loop {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read from pipe");
                input = input.to_string();
                if input == "" {
                    break;
                }
                content = format!("{}{}     ", content, input);
            }
        }
        println!("Content: {}", content);
        quickclip::put_content(
            &config.quickclip_url,
            &config.default_clipboard_id,
            &config.quicklip_username,
            &config.quicklip_password,
            content,
        )
        .await;
    } else if quickclip.mode == "get" || quickclip.mode == "g" {
        quickclip::get_content(
            &config.quickclip_url,
            &config.default_clipboard_id,
            &config.quicklip_username,
            &config.quicklip_password,
        )
        .await;
    } else {
        println!(
            "{}: Invalid Mode: {}",
            colors::red("Error"),
            colors::blue(quickclip.mode.as_str())
        )
    }
}

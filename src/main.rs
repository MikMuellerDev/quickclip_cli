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
    /// Method of interacting with QuickClip
    #[structopt(required = true)]
    mode: String,

    /// Content to set when using the --set /-s flag
    #[structopt(short = "i", long = "interactive")]
    interactive: bool,

    /// Content to set when using the --set /-s flag
    #[structopt(name = "CONTENT", required_if("mode", "set"))]
    content: Option<String>,
}

#[tokio::main]
async fn main() {
    let config: Config = config::read_config(CONFIG_FILE_PATH);
    println!("{:?}", &config);
    println!("Config file loaded {}.", colors::green("successfully"));

    let quickclip = QuickClip::from_args();

    if quickclip.mode == "set" {
        quickclip::put_content(
            &config.quickclip_url,
            &config.default_clipboard_id,
            &config.quicklip_username,
            &config.quicklip_password,
            quickclip.content.unwrap().to_string(),
        )
        .await;
    } else if quickclip.mode == "get" {
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

    // Experimental
    if quickclip.interactive {}
}

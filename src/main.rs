use crate::config::Config;
use std::process;
use structopt::StructOpt;
mod quickclip;
use std::env;
use std::io;
mod colors;
mod config;
mod file;
use file::read_file;

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

    /// If used, quickclip CLI will output only the fetched contents, making it useful for piping it into other processes.
    #[structopt(short = "o", long = "output")]
    output: bool,

    /// If used, quickclip CLI will append the content to the clip instead of replacing it.
    #[structopt(short = "a", long = "append")]
    append: bool,

    /// Content to push onto the clipboard when using the set mode
    #[structopt(name = "CONTENT")]
    content: Option<String>,

    /// Filename
    #[structopt(name = "FILENAME", short = "f", long = "file")]
    filename: Option<String>,

    /// Content to push onto the clipboard when using the set mode
    #[structopt(name = "CLIPBOARD ID", short = "c", long = "id")]
    optional_clip_id: Option<String>,
}

#[tokio::main]
async fn main() {
    let home_directory = env::var("HOME").unwrap_or_else(|e| {
        eprintln!("{}: retrieving home directory: {}", colors::red("Error"), e);
        process::exit(7)
    });
    let config_file_path: String = format!("{}/.config/quickclip.json", home_directory);
    let config: Config = config::read_config(config_file_path.as_str());

    let quickclip = QuickClip::from_args();

    let clipboard_id = quickclip
        .optional_clip_id
        .unwrap_or(config.default_clipboard_id);

    if quickclip.mode == "set" || quickclip.mode == "s" {
        let mut content: String;
        if !quickclip.input {
            content = quickclip
                .content
                .unwrap_or_else(|| {
                    eprintln!(
                        "{}: A content string is required when using set.",
                        colors::red("Error")
                    );
                    process::exit(1)
                })
                .to_string();
            if quickclip.append {
                content = format!(
                    "{}\n{}",
                    quickclip::fetch_content(
                        &config.quickclip_url,
                        &clipboard_id,
                        &config.quicklip_username,
                        &config.quicklip_password
                    )
                    .await,
                    content
                )
            }
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
                content = format!("{}{}", content, input);
            }
        }
        // println!("Content: {}", content);
        quickclip::put_content(
            &config.quickclip_url,
            &clipboard_id,
            &config.quicklip_username,
            &config.quicklip_password,
            content,
        )
        .await;
    } else if quickclip.mode == "get" || quickclip.mode == "g" {
        quickclip::get_content(
            &config.quickclip_url,
            &clipboard_id,
            &config.quicklip_username,
            &config.quicklip_password,
            !quickclip.output,
        )
        .await;
    } else if quickclip.mode == "getfile" || quickclip.mode == "gf" {
        let filename = quickclip.filename.unwrap_or_else(|| {
            eprintln!(
                "{}: A filename string is required when using getfile.",
                colors::red("Error")
            );
            process::exit(1)
        });
        file::write_file(
            quickclip::fetch_content(
                &config.quickclip_url,
                &clipboard_id,
                &config.quicklip_username,
                &config.quicklip_password,
            )
            .await,
            filename,
        )
    } else if quickclip.mode == "setfile" || quickclip.mode == "sf" {
        let filename = quickclip.filename.unwrap_or_else(|| {
            eprintln!(
                "{}: A filename string is required when using setfile.",
                colors::red("Error")
            );
            process::exit(1)
        });
        let file_content = read_file(filename);
        quickclip::put_content(
            &config.quickclip_url,
            &clipboard_id,
            &config.quicklip_username,
            &config.quicklip_password,
            file_content,
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

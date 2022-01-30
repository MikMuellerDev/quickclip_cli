use crate::colors;
// use reqwest::StatusCode;
use serde_json::json;
use std::process;

extern crate serde;
extern crate serde_json;
use serde::Deserialize;
// use serde::Serialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Clipboard {
    pub name: String,
    pub id: String,
    pub content: String,
    pub description: String,
    pub restricted: bool,
    pub refresh: bool,
    pub refresh_interval: u32,
    pub read_only: bool,
}

pub async fn put_content(
    url: &String,
    id: &String,
    username: &String,
    password: &String,
    content: String,
) {
    let body = json!({
        "Content": content,
    });

    let client = reqwest::Client::new();
    let res = client
        .put(format!(
            "{}/api/clips/edit/{}?username={}&password={}",
            url, id, username, password
        ))
        .json::<serde_json::Value>(&body)
        .send()
        .await
        .ok()
        .unwrap_or_else(|| {
            eprintln!("{}: QuickClip is unreachable", colors::red("Error"));
            process::exit(7);
        });

    let status_code = res.status();
    println!("Status Code: {}", status_code);
    if status_code == 200 {
        println!(
            "Contents of clipboard '{}' updated {}.",
            colors::blue(id),
            colors::green("successfully")
        )
    } else {
        println!(
            "Setting content {}. Response: {}",
            colors::red("failed"),
            status_code
        )
    }
}

pub async fn get_content(url: &String, id: &String, username: &String, password: &String) {
    let client = reqwest::Client::new();
    let res = client
        .get(format!(
            "{}/api/clip/{}?username={}&password={}",
            url, id, username, password
        ))
        .send()
        .await
        .ok()
        .unwrap_or_else(|| {
            eprintln!("{}: QuickClip is unreachable", colors::red("Error"));
            process::exit(7);
        });

    let status_code = res.status();
    if status_code == 200 {
        let response_text: String = res.text().await.ok().unwrap_or_else(|| {
            eprintln!("Server didn't send a text response.");
            process::exit(4);
        });

        // println!("{}", response_text);

        let clipboard: Clipboard = serde_json::from_str(response_text.as_str())
            .ok()
            .unwrap_or_else(|| {
                eprintln!("Parsing Server response failed, check your configuration.");
                process::exit(3)
            });
        // println!("{:?}", clipboard);
        // println!("Name: {}", clipboard.name);
        display_clipboard(clipboard)
    } else {
        println!(
            "Getting content {}. Response: {}",
            colors::red("failed"),
            status_code
        )
    }
}

pub fn display_clipboard(clipboard: Clipboard) {
    let text = format!(
        "
{}
\u{2502} Name:         \u{2502} {}
\u{2502} ID:           \u{2502} {} 
\u{2502} Description:  \u{2502} {}
\u{2502} Restricted:   \u{2502} {}
\u{2502} Refresh:      \u{2502} {}
\u{2502} Refresh Int:  \u{2502} {}
\u{2502} Read Only:    \u{2502} {}
{}
",
        get_times("\u{2500}".to_string(), 40),
        colors::blue(clipboard.name.as_str()),
        colors::blue(clipboard.id.as_str()),
        colors::blue(clipboard.description.as_str()),
        colors::blue(format!("{}", clipboard.restricted).as_str()),
        colors::blue(format!("{}", clipboard.refresh).as_str()),
        colors::blue(format!("{}", clipboard.refresh_interval).as_str()),
        colors::blue(format!("{}", clipboard.read_only).as_str()),
        get_times("\u{2500}".to_string(), 40),
    );
    println!("{}", text);
    // println!("{}", clipboard.content.replace("\n", "\n      "));

    let mut line_count: u16 = 0;
    print!("\n{}\u{2502}      ", add_padding(line_count));
    for char in clipboard.content.chars() {
        if char == '\n' {
            line_count += 1;
            print!("\n{}\u{2502}", add_padding(line_count));
        } else {
            print!("{}", char);
        }
    }
    println!()
}

fn add_padding(i: u16) -> String {
    if i < 10 {
        return format!("  {}", i);
    } else if i < 100 {
        return format!(" {}", i);
    } else {
        return format!("{}", i);
    }
}

fn get_times(string: String, count: u8) -> String {
    let mut text: String = string.to_string();
    for _ in 0..count {
        text = format!("{}{}", text, string);
    }
    return text;
}

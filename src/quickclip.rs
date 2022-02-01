use crate::config::Clipboard;
extern crate serde_json;
use serde_json::json;
extern crate serde;
use crate::colors;
use std::process;

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
    // println!("Status Code: {}", status_code);
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

pub async fn get_content(
    url: &String,
    id: &String,
    username: &String,
    password: &String,
    pretty: bool,
) {
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

        let clipboard: Clipboard = serde_json::from_str(response_text.as_str())
            .ok()
            .unwrap_or_else(|| {
                eprintln!("{}: Parsing Server response failed, check your configuration.", colors::red("Error"));
                process::exit(3)
            });
        if pretty {
            display_clipboard(clipboard)
        } else {
            println!("{}", clipboard.content)
        }
    } else {
        println!(
            "Getting clipboard {}. Response: {}",
            colors::red("failed"),
            status_code
        )
    }
}

fn display_clipboard(clipboard: Clipboard) {
    let text = format!(
        "
{}
\u{2502} Name:         \u{2502} {} {}
\u{2502} ID:           \u{2502} {} {}
\u{2502} Description:  \u{2502} {} {}
\u{2502} Restricted:   \u{2502} {} {}
\u{2502} Refresh:      \u{2502} {} {}
\u{2502} Refresh Int:  \u{2502} {} {}
\u{2502} Read Only:    \u{2502} {} {}
{}
",
        "\u{2500}".repeat(60),
        colors::blue(clipboard.name.as_str()),
        format!("{}\u{2502}", get_padding(clipboard.name.to_string())),
        colors::blue(clipboard.id.as_str()),
        format!("{}\u{2502}", get_padding(clipboard.id.to_string())),
        colors::blue(clipboard.description.as_str()),
        format!("{}\u{2502}", get_padding(clipboard.description.to_string())),
        colors::blue(format!("{}", clipboard.restricted).as_str()),
        format!("{}\u{2502}", get_padding(clipboard.restricted.to_string())),
        colors::blue(format!("{}", clipboard.refresh).as_str()),
        format!("{}\u{2502}", get_padding(clipboard.refresh.to_string())),
        colors::blue(format!("{}", clipboard.refresh_interval).as_str()),
        format!(
            "{}\u{2502}",
            get_padding(clipboard.refresh_interval.to_string())
        ),
        colors::blue(format!("{}", clipboard.read_only).as_str()),
        format!("{}\u{2502}", get_padding(clipboard.read_only.to_string())),
       "\u{2500}".repeat(60),
    );
    println!("{}", text);
    let content = clipboard.content;
    let mut line_count: u16 = 1;
    print!(
        "\n\x1b[2m\x1b[1;39m{: >3} \u{2502}\x1b[22m ",
        line_count
    );
    for char in content.chars() {
        if char == '\n' {
            line_count += 1;
            print!(
                "\n\x1b[2m\x1b[1;39m{: >3} \u{2502}\x1b[22m ",
                line_count
            );
        } else {
            print!("{}", char);
        }
    }
    println!("\n")
}

fn get_padding(string: String) -> String {
    let mut output = "".to_string();
    for _ in 0..41 - string.len() {
        output = format!(" {}", output);
    }
    return output;
}

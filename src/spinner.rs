use crate::colors;
use std::time::Duration;

const SPINNER_POSITIONS: [&str; 8] = ["⡇", "⣆", "⣤", "⣰", "⢸", "⠹", "⠛", "⠏"];

pub async fn start_spinner(text: &str) {
    println!("{}", text);
    print!("\x1b[1F");
    loop {
        for pos in SPINNER_POSITIONS {
            println!("{} {}\x1b[1F", colors::blue(pos), text);
            tokio::time::sleep(Duration::from_millis(70)).await;
        }
    }
}

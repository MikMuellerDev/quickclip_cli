use crate::colors;
use std::time::Duration;

const SPINNER_POSITIONS: [&str; 8] = ["⠏", "⠛", "⠹", "⢸", "⣰", "⣤", "⣆", "⡇"];

pub async fn start_spinner(text: &str) {
    eprintln!("{}", text);
    eprint!("\x1b[1F");
    loop {
        for pos in SPINNER_POSITIONS {
            eprintln!("{} {}\x1b[1F", text, colors::blue(pos));
            tokio::time::sleep(Duration::from_millis(70)).await;
        }
    }
}

pub fn blue(input: &str) -> String {
    format!("\x1b[1;34m{}\x1b[0m", input)
}

pub fn green(input: &str) -> String {
    format!("\x1b[1;32m{}\x1b[0m", input)
}

pub fn red(input: &str) -> String {
    format!("\x1b[1;31m{}\x1b[0m", input)
}

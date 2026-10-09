use super::super::Command;

fn execute(_text: &str) {
    match open::that("https://www.youtube.com") {
        Ok(_) => {},
        Err(e) => println!("Err: {}", e),
    }
}

pub fn create_command() -> Command {
    Command {
        name: "open_yotube",
        key_words: vec!["ютуб"],
        execute: execute,
    }
}


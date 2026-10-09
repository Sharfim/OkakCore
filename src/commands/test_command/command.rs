use super::super::Command;

fn execute(_text: &str) {
    println!("тест");
}

pub fn create_command() -> Command {
    Command {
        name: "test_command",
        key_words: vec!["тест"],
        execute: execute,
    }
}


use super::super::Command;
use crate::host_state::HostState;

fn execute(_text: &str, host_state: &HostState) {
    match host_state.voice.speak("тест") {
        Ok(_) => {},
        Err(e) => eprintln!("Err: {}", e),
    }
}

pub fn create_command() -> Command {
    Command {
        name: "test_command",
        key_words: vec!["тест"],
        execute: execute,
    }
}


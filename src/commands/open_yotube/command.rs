use super::super::Command;
use crate::host_state::HostState;

fn execute(_text: &str, _host_state: &HostState) {
    match open::that("https://www.youtube.com") {
        Ok(_) => {},
        Err(e) => {
            match _host_state.voice.speak("сэр, произошла ошибка") {
                Ok(_) => {},
                Err(e) => eprintln!("Err: {}", e),
            }
            eprintln!("Err: {}", e);
        },
    }

    match _host_state.voice.speak("приятного вам просмотра, сэр") {
        Ok(_) => {},
        Err(e) => eprintln!("Err: {}", e),
    }

}

pub fn create_command() -> Command {
    Command {
        name: "open_yotube",
        key_words: vec!["ютуб"],
        execute: execute,
    }
}


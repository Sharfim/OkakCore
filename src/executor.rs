use crate::commands::{all_commands, Command};
use crate::selector_command::voter_command;
use crate::host_state::HostState;
use std::collections::HashMap;

pub fn executor(text: &str, host_state: &HostState) {
    let vec_commands: Vec<Command> = all_commands();
    let mut key_words: HashMap<String, Vec<String>> = HashMap::new();

    for command in &vec_commands {
        let mut command_key_words: Vec<String> = Vec::new();

        for command_key_word in &command.key_words {
            command_key_words.push(command_key_word.to_string());
        }

        key_words.insert(command.name.to_string(), command_key_words);
    }

    let option_key_selected_command = voter_command(text, &key_words);
    //зачем: этот список может быть очень огромным, а дальше в коде он нам не нужен
    drop(key_words);
    let mut key_selected_command: String = "".to_string();

    match option_key_selected_command {
        Some(key) => key_selected_command = key,
        None => {
            println!("command not found");
            return;
        }
    }
    let mut selected_command: Option<fn(&str, &HostState)> = None;

    for command in &vec_commands {
        if command.name.to_string() == key_selected_command {
            selected_command = Some(command.execute);
            break;
        }
    }

    match selected_command {
        Some(f) => f(text, host_state),
        None => println!("Err: command not found"),
    }
}

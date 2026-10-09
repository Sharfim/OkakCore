mod open_yotube;
mod test_command;
use crate::host_state::HostState;

pub fn all_commands() -> Vec<Command> {
    vec![
        open_yotube::create_command(),
        test_command::create_command(),
    ]
}

pub struct Command {
    pub name: &'static str,          // имя команды
    pub key_words: Vec<&'static str>, // ключевые слова
    pub execute: fn(&str, &HostState),           // функция
}

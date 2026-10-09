use std::collections::HashMap;

pub fn voter_command(text: &str, key_words: &HashMap<String, Vec<String>>) -> Option<String> {
    let mut command_key: Option<String> = None;

    for (key, values) in key_words {
        for value in values {
            //прямое вхождение
            if text.to_string() == *value {
                command_key = Some(key.to_string());
                break;
            }
        }
        match command_key {
            Some(ref option_value) => break,
            None => {},
        }
    }

    command_key
}

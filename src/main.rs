mod commands;
mod selector_command;
mod executor;
mod host_state;
mod primitives;

use host_state::HostState;
use executor::executor;
use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Err: insufficient arguments");
        std::process::exit(1);
    }

    let text = &args[1];

    let state = HostState {
        voice: Box::new(ImplVoice {}),
    };

    executor(&text, &state);
}

struct ImplVoice {
    //...
}

impl primitives::Voice for ImplVoice {
    fn speak(&self, text: &str) -> Result<(), String> {
        println!("{}", text);
        Ok(())
    }

    fn listen(&self) -> Result<String, String> {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(bytes_read) => Ok(input.trim().to_string()),
            Err(e) => Err(e.to_string())
        }
    }
}

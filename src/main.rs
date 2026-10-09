mod commands;
mod selector_command;
mod executor;
use executor::executor;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Err: insufficient arguments");
        std::process::exit(1);
    }

    let text = &args[1];
    executor(&text);
}


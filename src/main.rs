use colored::*;
use std::{env, process};

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        let error_message = "Problem parsing arguments.".bright_red();
        eprintln!("{}: {}", error_message, err.red().bold().blink());
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

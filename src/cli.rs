use std::io::{self, Write};

pub struct Cli;

impl Cli {
    pub fn prompt_input(prompt: &str) -> String {
        loop {
            print!("{prompt}");
            if io::stdout().flush().is_err() {
                eprintln!("Failed to flush stdout.");
                continue;
            }

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                eprintln!("Failed to read input.");
                continue;
            }

            let input = input.trim();
            if input.is_empty() {
                eprintln!("Input cannot be empty.");
            } else {
                return input.to_string();
            }
        }
    }
}


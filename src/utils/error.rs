use colored::Colorize;
use std::io::Error;

pub trait ErrorPrinting {
    fn print_error(&self);
}

impl ErrorPrinting for Error {
    fn print_error(&self) {
        println!(
            "{} {}",
            "error".bright_red().to_string() + ":".bright_white().as_ref(),
            self.kind()
                .to_string()
                .split_whitespace()
                .map(|word| word[..1].to_uppercase() + &word[1..].to_lowercase().as_ref())
                .collect::<Vec<_>>()
                .join(" ")
                .bright_white()
        );
        println!();

        println!("{}", self.to_string());
    }
}

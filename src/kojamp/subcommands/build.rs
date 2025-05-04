use clap::Command;
use colored::Colorize;

pub fn cmd() -> Command {
    Command::new("build")
        .alias("b")
        .about("Being developed".bright_red().to_string())
}

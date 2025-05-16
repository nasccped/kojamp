use clap::Command;
use colored::Colorize;

pub fn cmd() -> Command {
    Command::new("build")
        .visible_alias("b")
        .about("Being developed".bright_red().to_string())
}

use clap::Command;
use colored::Colorize;

pub fn cmd() -> Command {
    Command::new("run")
        .visible_alias("r")
        .about("Being developed".bright_red().to_string())
}

use clap::Command;

pub fn cmd() -> Command {
    Command::new("run")
        .alias("r")
        .about("(Build and) run the project bytecode from the current directory")
}

use clap::Command;

pub fn cmd() -> Command {
    Command::new("build")
        .alias("b")
        .about("Compile the project source from the current directory into bytecode")
}

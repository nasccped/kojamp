use clap::Command;

pub fn cmd() -> Command {
    Command::new("build")
        .visible_alias("b")
        .about("Build the project bytecode (.class/.jar)")
}

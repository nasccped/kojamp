use clap::Command;

pub fn cmd() -> Command {
    Command::new("run")
        .visible_alias("r")
        .about("Run the compiled bytecode")
}

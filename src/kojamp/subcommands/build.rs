use clap::{Arg, ArgAction, Command};

pub fn cmd() -> Command {
    Command::new("build")
        .visible_alias("b")
        .about("Build the project bytecode (.class/.jar)")
        .args([Arg::new("polished")
            .long("polished")
            .short('p')
            .action(ArgAction::SetTrue)
            .required(false)
            .help("Build only the changed source, preserving the old .class(es)")])
}

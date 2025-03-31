mod commands;

use clap::Command;

const PROGRAM_NAME: &str = "kojamp";
const CURRENT_VERSION: &str = "0.0.1";
const AUTHOR: &str = "nasccped <pdbt.contact@gmail.com>";
const ABOUT: &str = "Some kind of basic 'n academic Java/Kotlin \"project-manager\" tool";

fn main() {
    let kojamp_man = Command::new(PROGRAM_NAME)
        .version(CURRENT_VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .subcommand(commands::hello::command())
        .subcommand(commands::bye::command())
        .get_matches();

    if let Some(refer) = kojamp_man.subcommand_matches("hello") {
        commands::hello::action(refer);
    } else if let Some(refer) = kojamp_man.subcommand_matches("bye") {
        commands::bye::action(refer);
    } else {
        println!("No subcommand provided");
    }
}

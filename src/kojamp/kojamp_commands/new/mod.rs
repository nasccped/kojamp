use clap::{ArgMatches, Command};

const COMMAND_NAME: &str = "new";
const COMMAND_ABOUT: &str = "Create a new \x1b[1;31mJava \x1b[37m/ \x1b[33mKotlin\x1b[0m project";

pub fn command_new() -> Command {
    Command::new(COMMAND_NAME).about(COMMAND_ABOUT)
}

pub fn action_new(mtc: ArgMatches) -> i32 {
    println!("This is printed when `new` command is called!");
    0
}

use clap::{Arg, ArgAction, Command};

pub fn command() -> Command {
    Command::new("new").about("Create a new Java/Kotlin project").args([
        Arg::new("name")
            .value_name("NAME")
            .action(ArgAction::Set)
            .required(false)
            .help("Specify the project's name")
            .index(1),
        Arg::new("path")
            .value_name("PATH")
            .action(ArgAction::Set)
            .long("path")
            .short('P')
            .required(false)
            .help("Specify the project's path (\x1b[93m[NAME]\x1b[0m in kebab case as default)"),
        Arg::new("type")
            .value_name("(J)ava|(K)otlin")
            .action(ArgAction::Set)
            .long("type")
            .short('T')
            .required(false)
            .help("Specify the project's language (Java or Kotlin)"),
        Arg::new("authors")
            .value_name("AUTHORS")
            .action(ArgAction::Set)
            .long("authors")
            .short('A')
            .required(false)
            .help("Specify the project's author(s) (slash '/' separated)")
            .num_args(1..),
        Arg::new("no-git")
            .long("no-git")
            .action(ArgAction::SetTrue)
            .required(false)
            .help("Disable git repository initialization"),
        Arg::new("prompt")
            .long("prompt")
            .action(ArgAction::SetTrue)
            .required(false)
            .help("Concise way to new kojamp based projects (try it by using \x1b[93m`kojamp new --prompt`\x1b[0m)")
    ])
}

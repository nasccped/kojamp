use clap::{Arg, ArgAction, ArgMatches, Command};

const COMMAND_NAME: &str = "new";
const COMMAND_ABOUT: &str = "Create a new Java/Kotlin project";

pub fn command_new() -> Command {
    Command::new(COMMAND_NAME).about(COMMAND_ABOUT).args([
        Arg::new("name")
            .value_name("NAME")
            .action(ArgAction::Set)
            .required(false)
            .help("Specify the project's name (numeric starting isn't allowed)")
            .index(1),
        Arg::new("path")
            .value_name("PATH")
            .action(ArgAction::Set)
            .long("path")
            .short('P')
            .required(false)
            .help("Specify the project's path (self name in kebab case as default)"),
        Arg::new("author")
            .value_name("AUTHOR")
            .action(ArgAction::Set)
            .long("author")
            .short('A')
            .required(false)
            .help("Specify the project's author(s) (comma separated)")
            .num_args(0..),
    ])
}

pub fn action_new(mtc: &ArgMatches) -> i32 {
    let (name, path, author): (_, _, Option<Vec<&String>>) = (
        mtc.get_one::<String>("name"),
        mtc.get_one::<String>("path"),
        mtc.get_many::<String>("author")
            .map(|vals| vals.clone().collect()),
    );

    println!("{:?}, {:?}, {:?}", name, path, author);

    0
}

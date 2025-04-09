use clap::{Arg, ArgAction, ArgMatches, Command};

const COMMAND_NAME: &str = "new";
const COMMAND_ABOUT: &str = "Create a new Java/Kotlin project";

pub fn command_new() -> Command {
    Command::new(COMMAND_NAME).about(COMMAND_ABOUT).args([
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
            .value_name(" java | kotlin ")
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
            .help("Specify the project's author(s) (comma separated)")
            .num_args(0..),
        Arg::new("no-git")
            .long("no-git")
            .action(ArgAction::SetFalse)
            .required(false)
            .help("Disable git repository initialization"),
        Arg::new("prompt")
            .long("prompt")
            .action(ArgAction::SetTrue)
            .required(false)
            .help("Concise way to new kojamp based projects (try it by using \x1b[93m`kojamp new --prompt`\x1b[0m)")
    ])
}

pub fn action_new(mtc: &ArgMatches) -> i32 {
    let (p_name, p_path, p_type, p_authors, git_repo, p_prompt): (
        Option<&String>,
        Option<&String>,
        Option<&String>,
        Option<Vec<&String>>,
        Option<&bool>,
        Option<&bool>,
    ) = (
        mtc.get_one("name"),
        mtc.get_one("path"),
        mtc.get_one("type"),
        mtc.get_many("authors").map(|vals| vals.clone().collect()),
        mtc.get_one("no-git"),
        mtc.get_one("prompt"),
    );

    for (k, v) in items {
        println!("{}: {:?}", k, v);
    }

    0
}

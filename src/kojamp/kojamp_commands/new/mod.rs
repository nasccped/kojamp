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
            .num_args(1..),
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
    let (name, path, project_type, authors, git_repo, prompt): (
        Option<String>,
        Option<String>,
        Option<String>,
        Option<Vec<String>>,
        Option<bool>,
        Option<bool>,
    ) = (
        mtc.get_one("name").cloned(),
        mtc.get_one("path").cloned(),
        mtc.get_one("type").cloned(),
        mtc.get_many("authors").map(|vals| vals.cloned().collect()),
        mtc.get_one("no-git").cloned(),
        mtc.get_one("prompt").cloned(),
    );

    let items = [
        ("Name", name),
        ("Path", path),
        ("Type", project_type),
        (
            "Author(s)",
            match authors {
                Some(vector) => Some(format!("{}{}{}", "[", vector.join(", "), "]")),
                _ => None,
            },
        ),
        (
            "Git Initialization:",
            if git_repo.unwrap_or(true) {
                Some("true".to_string())
            } else {
                Some("false".to_string())
            },
        ),
        (
            "Prompt Mode",
            if prompt.unwrap_or(true) {
                Some("true".to_string())
            } else {
                Some("false".to_string())
            },
        ),
    ];

    for (k, v) in items {
        println!("{}: {:?}", k, v);
    }

    0
}

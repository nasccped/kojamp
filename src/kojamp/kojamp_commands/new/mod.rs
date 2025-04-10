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
            .value_name("(J)ava|(K)otlin ")
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
        bool,
        bool,
    ) = (
        mtc.get_one("name").cloned(),
        mtc.get_one("path").cloned(),
        mtc.get_one("type").cloned(),
        mtc.get_many("authors").map(|vals| vals.cloned().collect()),
        match mtc.get_one::<bool>("no-git") {
            Some(&g) => g,
            _ => true,
        },
        match mtc.get_one::<bool>("prompt") {
            Some(&p) => p,
            _ => false,
        },
    );

    let prompt_conditions = [
        name.is_none(),
        path.is_none(),
        project_type.is_none(),
        authors.is_none(),
        git_repo,
    ];

    if prompt && !prompt_conditions.into_iter().all(|c| c) {
        println!("You \x1b[91mcan't\x1b[0m use `\x1b[93mkojamp new --prompt\x1b[0m` with other flags/args!");
        println!();
        println!("Try using one of the following:");
        println!("1) \x1b[92mkojamp new \x1b[93m[NAME]\x1b[0m + other flags (except --prompt)");
        println!("2) \x1b[92mkojamp new \x1b[93m--prompt\x1b[0m");
        println!();
        println!("Use `\x1b[92mkojamp new \x1b[93m--help\x1b[0m` for more details!");
        return 1;
    }

    0
}

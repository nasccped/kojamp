use clap::{Arg, ArgAction, Command};
use colored::Colorize;

pub fn cmd() -> Command {
    Command::new("new")
        .about("Create a Java/Kotlin project in a new directory")
        .args([
            Arg::new("name")
                .value_name("CAMEL_CASED_NAME")
                .action(ArgAction::Set)
                .required(true)
                .help("Specifies the project name")
                .index(1),
            Arg::new("kind")
                .long("kind")
                .short('k')
                .required(false)
                .value_name("JAVA|KOTLIN")
                .action(ArgAction::Set)
                .help("Specifies the project kind"),
            Arg::new("path")
                .long("path")
                .short('p')
                .required(false)
                .value_name("PATH")
                .action(ArgAction::Set)
                .help(format!(
                    "Specifies the project path {}",
                    "(project name in kebab case as default)"
                        .bright_black()
                        .italic()
                )),
            Arg::new("authors")
                .long("authors")
                .short('a')
                .required(false)
                .value_name("QUOTED_LIST")
                .action(ArgAction::Set)
                .help("Specifies the project authors"),
            Arg::new("no-git")
                .long("no-git")
                .required(false)
                .action(ArgAction::SetFalse)
                .help("No git repo initialization when starting new project"),
        ])
}

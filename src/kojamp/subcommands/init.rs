use clap::{Arg, ArgAction, Command};

pub fn cmd() -> Command {
    Command::new("init")
        .visible_alias("ini")
        .about("Create a Java/Kotlin project in the current directory")
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
            Arg::new("force")
                .long("force")
                .short('f')
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Force project initialization in a non empty dir"),
        ])
}

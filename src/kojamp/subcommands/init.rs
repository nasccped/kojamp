use clap::{Arg, ArgAction, Command};
use colored::Colorize;

pub fn cmd() -> Command {
    let description = format!(
        "Create a Java/Kotlin project in the current directory {}",
        "(Still being developed)".bright_red()
    );

    Command::new("init").alias("ini").about(description).args([
        Arg::new("name")
            .value_name("CAMEL_CASED_NAME")
            .action(ArgAction::Set)
            .required(true)
            .help("Specifies the project name")
            .index(1),
        Arg::new("kind")
            .long("kind")
            .short('K')
            .required(false)
            .value_name("JAVA|KOTLIN")
            .action(ArgAction::Set)
            .help("Specifies the project kind"),
        Arg::new("authors")
            .long("authors")
            .short('A')
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

use clap::{Arg, ArgAction, Command};
use colored::Colorize;

pub fn cmd() -> Command {
    let description = format!(
        "Create a Java/Kotlin project in the current directory {}",
        "(Still being developed)".bright_red()
    );

    Command::new("init").alias("ini").about(description).args([
        Arg::new("name")
            .value_name("NAME")
            .action(ArgAction::Set)
            .required(true)
            .help(format!(
                "Specify the project name {}",
                "(Should be CamelCased)".bright_red().italic()
            ))
            .index(1),
        Arg::new("kind")
            .long("kind")
            .short('K')
            .required(false)
            .value_name("JAVA|KOTLIN")
            .action(ArgAction::Set)
            .help(format!(
                "Specify the project kind {}",
                "(Java or Kotlin)".bright_black().italic()
            )),
        Arg::new("authors")
            .long("authors")
            .short('A')
            .required(false)
            .value_name("AUTHORS")
            .action(ArgAction::Set)
            .help(format!(
                "Specify the project authors {}",
                "(In quotes + comma separated)".bright_red().italic()
            )),
        Arg::new("no-git")
            .long("no-git")
            .required(false)
            .action(ArgAction::SetFalse)
            .help(format!(
                "Don't create a git repo when starting a new project"
            )),
    ])
}

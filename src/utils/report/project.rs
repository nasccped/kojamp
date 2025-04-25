use super::report_header;
use crate::globals::ERROR_BADGE;
use colored::Colorize;

pub fn name_is_invalid() {
    report_header(
        ERROR_BADGE.bright_red().bold(),
        "Couldn't create a new project due to invalid name!",
    );
    println!();

    println!("Accordingly to Java/Kotlin rules, a file/class name should");
    println!(
        "be {} and only alpha numeric. So, your project name",
        "CamelCased".bright_cyan()
    );
    println!("should start with an uppercase letter and have no special chars");
    println!("(accents, whitespaces, ... you get it).");
    println!();

    println!("Still, you can specify the path name by");
    println!(
        "using `{} {}` flag.",
        "--path".bright_green(),
        "<PATH_HERE>".bright_yellow()
    );
    println!();

    println!(
        "{} {}",
        "NOTE:".bright_cyan().italic(),
        "(`--path` flag only available for the `new` subcommand)".italic()
    )
}

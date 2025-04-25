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

pub fn kind_is_invalid(kind_value: &str) {
    report_header(
        ERROR_BADGE.bright_red().bold(),
        "Couldn't create a new project due to invalid project kind!",
    );
    println!();

    println!("This is a Java/Kotlin project manager, so these");
    println!(
        "kinds are expected, but {} was specified!",
        match kind_value {
            "" => format!("{}", "no kind".bright_red()),
            x => format!("`{}` kind", x.bright_red()),
        }
    );
    println!();

    println!("You can specify a project kind by");
    println!(
        "using the `{} {}`",
        "--kind".bright_green(),
        "<PROJECT_KIND>".bright_yellow()
    );
    println!();

    println!(
        "{} {}",
        "NOTE:".bright_cyan().italic(),
        format!(
            "{} {}{}{}{}{} {}",
            "<PROJECT_KIND>".bright_yellow().italic(),
            "can only be `".italic(),
            "Java".bright_blue().italic(),
            "` or `".italic(),
            "Kotlin".bright_blue().italic(),
            "`".italic(),
            "(No case sensitive)".bright_black().italic()
        )
    );
}

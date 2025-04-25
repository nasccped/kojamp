use super::report_header;
use crate::globals::ERROR_BADGE;
use colored::Colorize;

pub fn undefined_cur_dir() {
    report_header(
        ERROR_BADGE.bright_red().bold(),
        format!("Unable to get current path {}", "(returned err)"),
    );
    println!();

    println!("This only occurs due to the following reasons:");
    println!(
        "  {} The current directory doesn't exists",
        "a)".bright_cyan()
    );
    println!(
        "  {} You doesn't have sufficient permissions to access the current directory",
        "b)".bright_cyan()
    );
    println!();

    println!("Avoid creating projects in root directories, such as");
    println!(
        "`{}` on Windows or `{}` on Linux",
        "C:\\".bright_yellow().italic(),
        "/".bright_yellow().italic()
    );
}

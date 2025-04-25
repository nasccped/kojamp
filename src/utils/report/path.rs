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
        "C:\\".bright_yellow(),
        "/".bright_yellow()
    );
}

fn generic_invalid_path_project(path: Option<&str>) {
    report_header(
        ERROR_BADGE.bright_red().bold(),
        "Couldn't create a project due to invalid path!",
    );
    println!();

    println!(
        "The path {}",
        path.unwrap_or("[UNDEFINED PATH]").bright_red()
    );
    println!("returned fail when doing validation tests.");
}

pub fn invalid_path_when_new(path: Option<&str>) {
    generic_invalid_path_project(path);
    println!();

    println!("This can occur due to some reasons:");

    let reasons = [
        format!(
            "You've set a compound path {}",
            "(Like 'this/path')".bright_black()
        ),
        format!(
            "You've used `{}`/`{}` in path value {}",
            "CUR_DIR<.>".bright_blue(),
            "PREV_DIR<..>".bright_blue(),
            "(like 'this/..')".bright_black()
        ),
        "The path already exists".to_string(),
    ];

    for (r, ind) in reasons.into_iter().zip('a'..='c') {
        let col_ind = [ind, ')'].into_iter().collect::<String>().bright_cyan();
        println!("  {} {}", col_ind, r);
    }
    println!();

    println!("Also, this can occur if you don't specify a path name!");
    println!("The program build a kebab-case path based on your");
    println!(
        "project name, so, if you create a new project called `{}`",
        "CoolJava".bright_green()
    );
    println!(
        "and there's a directory called `{}`, the program",
        "cool-java".bright_yellow()
    );
    println!("will fail!");
    println!();

    println!(
        "You can use `{} {} {} {}`",
        "kojamp new".bright_green(),
        "<PROJECT_NAME>".bright_yellow(),
        "--path".bright_green(),
        "<PATH_NAME>".bright_yellow()
    );
    println!("in this case.");
}

pub fn invalid_path_when_init(path: Option<&str>) {
    generic_invalid_path_project(path);
    println!();

    println!("If you're seeing this message, you're probably");
    println!("near to the root path.");
    println!("Avoid creating projects here. It can conflict with");
    println!("The path validation tests!");
}

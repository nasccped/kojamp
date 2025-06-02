use super::super::consts::program::PROGRAM_REPO_URL;
use super::IntoReasons;
use crate::utils::string::StringTransformation;
use colored::Colorize;
use std::path::{Path, PathBuf};

pub fn invalid_cur_dir() -> String {
    let function_name = format!(
        "{}{}{}{}{}{}",
        "std".bright_red(),
        "::".bright_white(),
        "env".bright_red(),
        "::".bright_white(),
        "current_dir".bright_red(),
        "()".bright_white(),
    );

    let mut reasons = [
        String::from("The current path doesn't exists"),
        format!(
            "You doesn't have enough permissions {}",
            "(no sudo or admin)".bright_black()
        ),
    ]
    .into_reasons();

    let mut next = || {
        reasons
            .next()
            .unwrap_or("UNEXPECTED UNWRAPING".bright_red().to_string())
    };

    format!(
        "\
        The `{}` function returned an error\n\
        \n\
        The reason may be:\n\
        {}\n\
        {}",
        function_name,
        next(),
        next(),
    )
}

pub fn invalid_project_name<T: AsRef<str>>(name: T) -> String {
    let name = name.as_ref();
    let fixed_name = name.to_valid_camel_case();

    let mut reasons = [
        format!("Should be {}", "camel case".bright_cyan()),
        format!("Have no {}", "special chars".bright_cyan()),
        format!("Start with a letter {}", "(A-Z)".bright_cyan()),
    ]
    .into_reasons();

    let mut next = || {
        reasons
            .next()
            .unwrap_or("UNEXPECTED UNWRAPING".bright_red().to_string())
    };

    format!(
        "\
        The `{}` project name isn't allowed\n\
        \n\
        The project name:\n\
        {}\n\
        {}\n\
        {}\n\
        \n\
        Consider using `{}` instead",
        name.bright_red(),
        next(),
        next(),
        next(),
        fixed_name.bright_green()
    )
}

pub fn could_not_read_dir_content() -> String {
    format!(
        "\
        This can be due to {} issues",
        "permission".bright_cyan()
    )
}

pub fn non_empty_dir_initializing() -> String {
    let non_empty = "non empty".bright_red();
    let force = "--force".bright_yellow();
    let note = "note".bright_cyan();

    format!(
        "\
        You're trying to initialize the project \n\
        in a {} directory\n\
        \n\
        You can still force the initialization by \n\
        using the `{}` flag. All the files at current\n\
        dir will be preserved\n\
        \n\
        {}: `src/`, `Kojamp.toml` and `out/` shouldn't be present",
        non_empty, force, note
    )
}

pub fn could_not_create_dir_file(target: &str) -> String {
    format!(
        "\
        Error trying to create `{}`\n\
        Probably due to {} issue",
        target.bright_red(),
        "memory".bright_cyan()
    )
}

pub fn invalid_project_kind(kind_name: &str) -> String {
    let java = "Java".bright_blue();
    let kotlin = "Kotlin".bright_blue();
    let kind_name = kind_name.bright_blue();

    format!(
        "\
        `{}` or `{}` kind was expected\n\
        but `{}` was found",
        java,
        kotlin,
        kind_name.bright_red()
    )
}

pub fn invalid_project_path(path: &PathBuf) -> String {
    let path = format!("{:?}", path).bright_red();
    let path_flag = "--path".bright_yellow();

    let mut reasons = [
        String::from("You're near to the root of your file storage"),
        format!(
            "The path already exists (when {} a new project)",
            "creating".bright_cyan()
        ),
        format!(
            "The path doesn't exists (when {} a new project)",
            "initializing".bright_cyan()
        ),
    ]
    .into_reasons();

    let mut next = || {
        reasons
            .next()
            .unwrap_or("UNEXPECTED UNWRAPING".bright_red().to_string())
    };

    format!(
        "\
        The `{}` path\n\
        returned fail when testing validation\n\
        \n\
        This can be due to some reasons like:\n\
        {}\n\
        {}\n\
        {}\n\
        \n\
        If you're creating a new project, consider specify another\n\
        path by using the `{}` flag",
        path,
        next(),
        next(),
        next(),
        path_flag
    )
}

pub fn could_not_initialize_git_repo() -> String {
    let mut reasons = [
        format!("You doesn't have {} program", "git".bright_cyan()),
        format!("{} issues, though", "Mem.".bright_red()),
    ]
    .into_reasons();

    let mut next = || {
        reasons
            .next()
            .unwrap_or("UNEXPECTED UNWRAPING".bright_red().to_string())
    };

    format!(
        "\
        This can be due the following reasons:\n\
        {}\n\
        {}",
        next(),
        next()
    )
}

pub fn main_app_undefined_error() -> String {
    format!(
        "\
        This message serves to alert that the program has\n\
        fallen into an {}\n\
        \n\
        Please, consider opening an {} at {}\n\
        \n\
        Describe your steps to get here",
        "unexpected behavior".bright_red(),
        "issue".bright_cyan(),
        PROGRAM_REPO_URL
    )
}

pub fn kojamp_toml_not_found() -> String {
    format!(
        "\
        {} file wasn't found.\n\
        Make sure you're at the {}",
        "Kojamp.toml".bright_cyan(),
        "right path".bright_green()
    )
}

pub fn empty_message() -> String {
    String::new()
}

pub fn toml_file_could_not_be_read() -> String {
    format!(
        "\
    It {} but {}",
        "exists".bright_green(),
        "couldn't be read".bright_red()
    )
}

pub fn unreadable_src_content(path: &Path) -> String {
    format!(
        "\
        You're trying to read `{}` dir entries but get\n\
        fail at a `{}`
        ",
        "src".bright_green(),
        format!("{:?}", path).bright_red()
    )
}

pub fn theres_no_files_for_the_given_project_kind(kind: &str) -> String {
    format!(
        "\
        At least one {} file should be present in `{}` dir",
        kind.bright_cyan(),
        "src".bright_green()
    )
}

pub fn main_project_file_is_not_present(file_name: String) -> String {
    format!(
        "\
        The `{}` file was expected but\n\
        it couldn't be found!",
        file_name.bright_green()
    )
}

pub fn successfully_compiled(file_count: usize) -> String {
    format!(
        "\
        {} file(s) successfully compiled (`{}` dir)",
        file_count,
        "out".bright_yellow()
    )
}

pub fn output_file_doesnt_exists(file_path: &Path) -> String {
    format!(
        "\
        Trying to find `{}` file but it doesn't exists.\n\
        Consider using `{}` before",
        file_path.to_string_lossy().bright_green(),
        "kojamp run".bright_green()
    )
}

pub fn could_not_generate_output_file_path(name: &str, kind: &str) -> String {
    format!(
        "\
        Couldn't generate the output file path for\n\
        `{}` name and `{}` kind.\n\
        \n\
        They're probably unvalid fields at `{}`",
        name.bright_green(),
        kind.bright_green(),
        "Kojamp.toml".bright_green()
    )
}

use super::util::IntoReasons;
use crate::utils::string::StringTransformation;
use colored::Colorize;
use std::path::PathBuf;

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

pub fn invalid_project_name<T>(name: T) -> String
where
    T: AsRef<str>,
{
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

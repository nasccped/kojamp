use crate::utils::string::StringTransformation;
use colored::Colorize;

pub fn invalid_cur_dir() -> String {
    format!(
        "\
        The `{}` function returned an error!\n\
        \n\
        The reason may be:\n\
        ... {} The current path doesn't exists\n\
        ... {} You doesn't have enough permissions {}",
        format!(
            "{}{}{}{}{}{}",
            "std".bright_red(),
            "::".bright_white(),
            "env".bright_red(),
            "::".bright_white(),
            "current_dir".bright_red(),
            "()".bright_white()
        ),
        "a)".bright_cyan(),
        "b)".bright_cyan(),
        "(no sudo or admin)".bright_black()
    )
}

pub fn invalid_project_name<T>(name: T) -> String
where
    T: AsRef<str>,
{
    let name = name.as_ref();
    let fixed_name = name.to_valid_camel_case();

    format!(
        "\
        The `{}` project name isn't allowed!\n\
        \n\
        The project name:\n\
        ... {} Should be {}\n\
        ... {} Have no {}\n\
        ... {} Start with a letter {}\n\
        \n\
        Consider using `{}` instead!",
        name.bright_red(),
        "a)".bright_cyan(),
        "Camel Case".bright_cyan(),
        "a)".bright_cyan(),
        "special chars".bright_cyan(),
        "c)".bright_cyan(),
        "(A-Z)".bright_cyan(),
        fixed_name.bright_green()
    )
}

pub fn could_not_read_dir_content() -> String {
    format!(
        "\
        This can be due to {} issues!",
        "no permission".bright_cyan()
    )
}

pub fn non_empty_dir_initializing() -> String {
    format!(
        "\
        You're trying to initialize the project \n\
        in a '{}' directory.\n\
        \n\
        You can still force the initialization by \n\
        using the `{}` flag. All the files at current\n\
        dir will be preserved!\n\
        \n\
        {}: `src/` and `Kojamp.toml` shouldn't be present",
        "non empty".bright_red(),
        "--force".bright_yellow(),
        "note".bright_cyan(),
    )
}

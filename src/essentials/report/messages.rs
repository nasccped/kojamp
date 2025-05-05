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
        The `{}` project name isn't allowed!\n\n\
        The project name should be {} and have no {}.\n\
        {}{} numeric start isn't allowed too\n\
        \n\
        Consider using `{}` instead!",
        name.bright_red(),
        "Camel Case".bright_cyan(),
        "Special Chars".bright_cyan(),
        "note".bright_cyan(),
        ":".bright_white(),
        fixed_name.bright_green()
    )
}

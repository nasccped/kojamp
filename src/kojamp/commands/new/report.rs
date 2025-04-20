use crate::{
    utils::io::{IOReporting, ReportStatus},
    vec_dispbox,
};
use std::borrow::Cow;

type CowAlias<'a, 'b> = &'a Cow<'b, str>;

pub fn prompt_not_allowed() {
    let io_report = IOReporting::new(
        ReportStatus::Err,
        Some("On `new --prompt`"),
        vec_dispbox![
            format!(
                "You probably typed something like: {},",
                format!(
                    "{}`kojamp new {}<BUNCH_OF_ARGS/FLAGS>{} --prompt`{}",
                    "\x1b[92m", "\x1b[90m", "\x1b[92m", "\x1b[0m"
                )
            ),
            "and this approach isn't allowed!",
            "",
            format!(
                "{}{}{} will start a Ask-Answer scheme to set the required",
                "\x1b[92m", "`kojamp new --prompt`", "\x1b[0m"
            ),
            "data and build a new project. Passing args/flags can generate an",
            "unexpected behavior!",
            "",
            "For everything to work properly, consider using:",
            format!(
                "  {}a) {}{} {}(for prompt mode generating){}",
                "\x1b[96m", "\x1b[92m", "`kojamp new --prompt`", "\x1b[90m", "\x1b[0m"
            ),
            format!(
                "  {}b) {} (for default mode generating){}",
                "\x1b[96m",
                format!(
                    "{}`kojamp new {}<YOUR_PROJECT_NAME>{} -T {}<YOUR_PROJECT_TYPE>{}`{}",
                    "\x1b[92m", "\x1b[93m", "\x1b[92m", "\x1b[93m", "\x1b[92m", "\x1b[90m"
                ),
                "\x1b[0m"
            ),
            "",
            format!(
                "{}<YOUR_PROJECT_TYPE>{} value can only be: {} {}{}{}",
                "\x1b[93m",
                "\x1b[0m",
                ["J", "Java", "K", "Kotlin"]
                    .into_iter()
                    .map(|elem| format!("{}{}{}", "\x1b[96m", elem, "\x1b[0m"))
                    .collect::<Vec<_>>()
                    .join(", "),
                "\x1b[90m",
                "(No case sensitive)",
                "\x1b[0m"
            ),
        ],
    );
    io_report.print_content();
}

pub fn invalid_name(name: CowAlias) {
    let io_report = IOReporting::new(
        ReportStatus::Err,
        Some("Trying to build a project with an invalid name"),
        vec_dispbox![
            format!("You can't build a project with the `{}` name due", name),
            "to the following reasons:",
            "",
            format!(
                "  {}a){} Your project name also defines the main func file name",
                "\x1b[96m", "\x1b[0m"
            ),
            format!(
                "  {}b){} Java/Kotlin have a {}Language Convetion{} for file name: CamelCase",
                "\x1b[96m", "\x1b[0m", "\x1b[96m", "\x1b[0m"
            ),
            "",
            "Basically, creating a new project that doesn't follow",
            format!(
                "this {}Language Conventions{} can result in a {}bug/error{} at compile",
                "\x1b[96m", "\x1b[0m", "\x1b[91m", "\x1b[0m"
            ),
            format!(
                "or run time! {}(Avoid using special chars/whitespaces too){}",
                "\x1b[90m", "\x1b[0m"
            )
        ],
    );
    io_report.print_content();
}

pub fn invalid_project_type(p_type: CowAlias) {
    let io_report = IOReporting::new(
        ReportStatus::Err,
        Some("Trying to build a project with an invalid type"),
        vec_dispbox![
            "This is a Java/Kotlin project manager, so a Java/Kotlin",
            format!("project type is expected, but the `{}`", p_type.as_ref()),
            "type was found!",
            "",
            "To specify an acceptable project type, you can",
            format!(
                "use the {}`--type {}<YOUR_PROJECT_TYPE>{}`{}, flag/arg.",
                "\x1b[3;92m", "\x1b[93m", "\x1b[92m", "\x1b[0m"
            ),
            "",
            format!(
                "{}NOTE:{} the project type arg can only",
                "\x1b[3;96m", "\x1b[0m",
            ),
            format!(
                "be {} or {}. {}",
                ["J", "Java", "K"]
                    .into_iter()
                    .map(|word| format!("{}{}{}", "\x1b[3;96m", word, "\x1b[0m"))
                    .collect::<Vec<_>>()
                    .join(", "),
                "\x1b[3;96mKotlin\x1b[0m",
                "\x1b[3;90m(No case sensitive)\x1b[0m"
            )
        ],
    );
    io_report.print_content();
}

pub fn invalid_authors(authors: CowAlias) {
    let io_report = IOReporting::new(
        ReportStatus::Err,
        Some(format!(
            "Trying to build a project with invalid authors: {}",
            authors.as_ref()
        )),
        vec_dispbox![
            "Authors aren't required for project creating!",
            "This error only occurs when you try to set authors with",
            format!(
                "invalid chars such as {}.",
                ['?', '/', '!', ',']
                    .into_iter()
                    .map(|c| format!("{}'{}'{}", "\x1b[91m", c, "\x1b[0m"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            "",
            format!(
                "You can create a project with {}no authors{} or avoid using",
                "\x1b[96m", "\x1b[0m"
            ),
            "The invalid chars!"
        ],
    );
    io_report.print_content();
}

pub fn no_args_or_flags() {
    let io_report = IOReporting::new(
        ReportStatus::Warn,
        Some("No args founded"),
        vec_dispbox![
            "You've probably typed something like",
            format!(
                "{}`kojamp new`{}. This won't do anything.",
                "\x1b[92m", "\x1b[0m"
            ),
            "",
            "If you want to check the help panel, consider using",
            format!(
                "{}`kojamp new --help`{}. If you want to run the subcommand,",
                "\x1b[92m", "\x1b[0m"
            ),
            format!(
                "consider using {}`kojamp new {}<PROJECT_NAME>{} --type {}<PROJECT_TYPE>{}`{}.",
                "\x1b[92m", "\x1b[93m", "\x1b[92m", "\x1b[93m", "\x1b[92m", "\x1b[0m"
            )
        ],
    );
    io_report.print_content();
}

pub fn invalid_cur_path() {
    let io_report = IOReporting::new(
        ReportStatus::Err,
        Some("Trying to create a project in an invalid directory"),
        vec_dispbox![
            format!(
                "Obtained `{}None{}` when trying to access the current working directory",
                "\x1b[91m", "\x1b[0m"
            ),
            "",
            "It car occur by some reasons like:",
            "",
            format!(
                "  {}a){} The current directory doesn't exists {}(You probably removed it){}",
                "\x1b[96m", "\x1b[0m", "\x1b[90m", "\x1b[0m"
            ),
            format!(
                "  {}a){} There's insufficient access permissions {}(Common at unix envs){}",
                "\x1b[96m", "\x1b[0m", "\x1b[90m", "\x1b[0m"
            )
        ],
    );
    io_report.print_content();
}

pub fn invalid_project_abs_path(path: CowAlias) {
    let io_report = IOReporting::new(
        ReportStatus::Err,
        Some(format!(
            "Couldn't create a project for the path: `{}`",
            path
        )),
        vec_dispbox![
            "When trying to get the project's absolute path, it",
            format!("returned {}`None`{}.", "\x1b[91m", "\x1b[0m"),
            "",
            "This may occur due to an invalid path char.",
            format!(
                "Avoid using `{}/{}` and special chars",
                "\x1b[91m", "\x1b[0m"
            )
        ],
    );
    io_report.print_content()
}

pub fn path_already_exists(path: CowAlias) {
    let io_report = IOReporting::new(
        ReportStatus::Err,
        Some(format!("Project path `{}` already exists", path)),
        vec_dispbox![
            "Couldn't create the project directory because it",
            "already exists!",
            "",
            "When creating a new project, the path will be automatically",
            "set to the project name in kebab case, or you can specify a path",
            format!(
                "by using the {}`--path {}<VALUE>{}`{} flag/args.",
                "\x1b[92m", "\x1b[93m", "\x1b[92m", "\x1b[0m"
            )
        ],
    );
    io_report.print_content();
}

pub fn permission_denied() {
    let io_report = IOReporting::new(
        ReportStatus::Err,
        Some("Permission denied for directory creation"),
        vec_dispbox![
            "This is a common behavior at unix-like systems.",
            "",
            "You're trying to run the command without being sudo or root"
        ],
    );
    io_report.print_content();
}

pub fn undefined_error() {
    let io_report = IOReporting::new(
        ReportStatus::Err,
        Some("Undefined Error"),
        vec_dispbox![
            "This is an undefined error when trying to create a new dir/file.",
            "",
            "This message is being printed because all errors have been",
            "covered and this is an unexpected one",
            "",
            format!(
                "Please, consider opening an issue at {}https://github.com/nasccped/kojamp{}",
                "\x1b[92m", "\x1b[0m"
            ),
            "Describing the steps to get this behavior!"
        ],
    );
    io_report.print_content();
}

use crate::{
    utils::io::{IOReporting, ReportStatus},
    vec_dispbox,
};
use std::borrow::Cow;

type CowAlias<'a, 'b> = &'a Cow<'b, str>;

pub fn prompt_not_allowed() {
    let io_report: IOReporting = IOReporting::new(
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
    let io_report: IOReporting = IOReporting::new(
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

pub fn invalid_project_type(p_type: CowAlias) {}

pub fn invalid_authors(authors: CowAlias) {
    println!("The current authors are invalid");
}

use std::fmt;

use crate::{
    utils::io::{IOReporting, ReportStatus},
    vec_dispbox,
};

pub fn prompt_not_allowed() {
    let io_report: IOReporting = IOReporting::new::<_, &str>(
        ReportStatus::Err,
        Some("FAIL ON `new --prompt`"),
        None,
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
                "{}a) {}{} {}(for prompt mode generating){}",
                "\x1b[96m", "\x1b[92m", "`kojamp new --prompt`", "\x1b[90m", "\x1b[0m"
            ),
            format!(
                "{}b) {} (for default mode generating){}",
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

pub fn invalid_name() {
    println!("The current name is invalid");
}

pub fn invalid_project_type() {
    println!("The current project type is invalid");
}

pub fn invalid_authors() {
    println!("The current authors are invalid");
}

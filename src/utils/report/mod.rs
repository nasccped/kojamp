pub mod path;
pub mod project;

use colored::Colorize;
use core::fmt;

fn report_header<T, U>(tag: T, text_message: U)
where
    T: Colorize + fmt::Display,
    U: AsRef<str>,
{
    let msg = text_message.as_ref();
    println!(
        "{tag}{} {}",
        ":".bright_white().bold(),
        msg.bright_white().bold()
    );
}

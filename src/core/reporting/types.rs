use super::super::consts::report_badges::{ERROR_BADGE, SUCCESS_BADGE, WARNING_BADGE};
use colored::Colorize;
use std::fmt;
use std::rc::Rc;

pub enum ReportType {
    Error,
    Warning,
    Success,
}

pub struct KojampReport {
    report_type: ReportType,
    title: Rc<str>,
    content: Rc<str>,
}

impl KojampReport {
    pub fn new<T, U>(report_type: ReportType, title: T, content: U) -> Self
    where
        T: AsRef<str>,
        U: AsRef<str>,
    {
        let title = Rc::from(title.as_ref());
        let content = Rc::from(content.as_ref());

        Self {
            report_type,
            title,
            content,
        }
    }
}

impl fmt::Display for KojampReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{} {}\n\n{}",
            match self.report_type {
                ReportType::Error => ERROR_BADGE.bright_red(),
                ReportType::Warning => WARNING_BADGE.bright_yellow(),
                _ => SUCCESS_BADGE.bright_green(),
            },
            ":".bright_white(),
            self.title.bright_white(),
            self.content
        )
    }
}

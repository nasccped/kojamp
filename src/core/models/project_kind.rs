use super::super::contracts::IsValid;
use clap::ArgMatches;
use std::rc::Rc;

pub enum ProjectKind {
    Java,
    Kotlin,
    Undefined,
    Invalid(Rc<str>),
}

impl From<&ArgMatches> for ProjectKind {
    fn from(value: &ArgMatches) -> Self {
        let value: &String = if let Some(v) = value.get_one("kind") {
            v
        } else {
            return Self::Undefined;
        };

        match value.trim().to_lowercase().as_ref() {
            "java" => Self::Java,
            "kotlin" => Self::Kotlin,
            _ => Self::Invalid(Rc::from(value.as_ref())),
        }
    }
}

impl IsValid for ProjectKind {
    fn is_valid(&self) -> bool {
        matches!(self, Self::Java | Self::Kotlin)
    }
}

impl<'a> From<&'a ProjectKind> for &'a str {
    fn from(value: &'a ProjectKind) -> Self {
        match value {
            ProjectKind::Java => "java",
            ProjectKind::Kotlin => "kotlin",
            ProjectKind::Undefined => "[N/A]",
            ProjectKind::Invalid(x) => x,
        }
    }
}

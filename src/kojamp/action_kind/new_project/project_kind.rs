use std::rc::Rc;

use clap::ArgMatches;

pub enum ProjectKind {
    Java,
    Kotlin,
    NotGiven,
    Invalid(Rc<str>),
}

impl ProjectKind {
    pub fn is_valid(&self) -> bool {
        matches!(self, ProjectKind::Java | ProjectKind::Kotlin)
    }
}

impl From<&ArgMatches> for ProjectKind {
    fn from(value: &ArgMatches) -> Self {
        let kind = if let Some(k) = value.get_one::<String>("kind") {
            k
        } else {
            return Self::NotGiven;
        };

        match kind.to_lowercase().trim().as_ref() {
            "java" => Self::Java,
            "kotlin" => Self::Kotlin,
            _ => Self::Invalid(Rc::from(kind.as_str())),
        }
    }
}

impl<'a> From<&'a ProjectKind> for &'a str {
    fn from(value: &'a ProjectKind) -> Self {
        match value {
            ProjectKind::Java => "java",
            ProjectKind::Kotlin => "kotlin",
            ProjectKind::Invalid(x) => x.as_ref(),
            _ => "[N/A]",
        }
    }
}

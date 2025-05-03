use clap::ArgMatches;

pub enum ProjectKind {
    Java,
    Kotlin,
    Invalid,
}

impl ProjectKind {
    pub fn is_valid(&self) -> bool {
        matches!(self, ProjectKind::Java | ProjectKind::Kotlin)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ProjectKind::Java => "java",
            ProjectKind::Kotlin => "kotlin",
            ProjectKind::Invalid => "INVALID",
        }
    }
}

impl From<&ArgMatches> for ProjectKind {
    fn from(value: &ArgMatches) -> Self {
        let kind = if let Some(k) = value.get_one::<String>("kind") {
            k
        } else {
            return Self::Invalid;
        };

        match kind.to_lowercase().trim().as_ref() {
            "java" => Self::Java,
            "kotlin" => Self::Kotlin,
            _ => Self::Invalid,
        }
    }
}

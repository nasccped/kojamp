use crate::utils::strings::ToTitleCase;
use clap::ArgMatches;

#[derive(Debug)]
enum ProjectType {
    Java,
    Kotlin,
    #[allow(dead_code)]
    Undefined(String),
}

impl ProjectType {
    fn from_string(input: String) -> Self {
        let input = input.to_lowercase();

        match input.as_str() {
            "java" | "j" => Self::Java,
            "kotlin" | "k" => Self::Kotlin,
            else_case => Self::Undefined(else_case.to_string()),
        }
    }

    #[allow(dead_code)]
    fn is_valid(&self) -> bool {
        match self {
            ProjectType::Kotlin | ProjectType::Java => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Project {
    name: Option<String>,
    path: Option<String>,
    project_type: Option<ProjectType>,
    authors: Option<Vec<String>>,
    no_git: bool,
    prompt: bool,
}

impl Project {
    pub fn from_match(matching: &ArgMatches) -> Self {
        let name: Option<String> = matching.get_one("name").cloned();
        let path: Option<String> = matching.get_one("path").cloned();
        let project_type: Option<ProjectType> = match matching.get_one::<String>("type").cloned() {
            Some(val) => Some(ProjectType::from_string(val)),
            None => None,
        };
        let mut authors: Option<Vec<String>> = matching
            .get_many("authors")
            .map(|vector| vector.cloned().collect());
        if authors.is_some() {
            authors = Some(
                authors
                    .unwrap()
                    .into_iter()
                    .map(|val| val.to_title_case())
                    .collect::<Vec<String>>()
                    .join(" ")
                    .split("/")
                    .filter(|name| !name.is_empty())
                    .map(|name| name.trim().to_string())
                    .collect(),
            );
        }
        let no_git = match matching.get_one("no-git") {
            Some(&cond) => cond,
            _ => false,
        };
        let prompt = match matching.get_one("prompt") {
            Some(&cond) => cond,
            _ => false,
        };

        Self {
            name,
            path,
            project_type,
            authors,
            no_git,
            prompt,
        }
    }

    pub fn prompt_called(&self) -> bool {
        self.prompt
    }

    pub fn prompt_allowed(&self) -> bool {
        [
            self.name.is_none(),
            self.path.is_none(),
            self.project_type.is_none(),
            self.authors.is_none(),
            !self.no_git,
        ]
        .iter()
        .all(|&condition| condition)
    }

    pub fn prompt_mode(&mut self) {}
}

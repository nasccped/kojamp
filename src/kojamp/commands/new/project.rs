use crate::utils::strings::{StringChecker, ToTitleCase};
use clap::ArgMatches;

#[derive(Debug, Clone)]
pub enum ProjectType {
    Java,
    Kotlin,
    #[allow(dead_code)]
    Undefined(String),
}

impl ProjectType {
    fn from_string(input: String) -> Self {
        let lower_input = input.to_lowercase();

        match lower_input.as_str() {
            "java" | "j" => Self::Java,
            "kotlin" | "k" => Self::Kotlin,
            _ => Self::Undefined(input),
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            ProjectType::Undefined(_) => false,
            _ => true,
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

    pub fn name_is_valid(&self) -> bool {
        let project_name = self.name.clone();

        if project_name.is_none() {
            return false;
        }

        let project_name = project_name.unwrap();

        if project_name.is_empty() {
            return false;
        }

        if !('A'..='Z').contains(&project_name.chars().next().unwrap()) {
            return false;
        }

        if project_name.contains(' ') {
            return false;
        }

        let char_range: String = {
            let mut final_result: String = String::new();

            let all_cases: [String; 3] = [
                ('A'..='Z').collect(),
                ('a'..='z').collect(),
                ('0'..='9').collect(),
            ];

            all_cases
                .into_iter()
                .for_each(|string| final_result.extend(string.chars()));

            final_result
        };

        if !StringChecker::chars_in_range(&project_name, char_range) {
            false
        } else {
            true
        }
    }

    pub fn get_type(&self) -> Option<ProjectType> {
        self.project_type.clone()
    }

    pub fn type_is_valid(&self) -> bool {
        let project_clone = self.project_type.clone();
        match project_clone {
            None => false,
            Some(x) => x.is_valid(),
        }
    }
}

#[cfg(test)]
mod kojamp_commands_new_project {

    use super::*;
    use crate::kojamp::builder;

    #[test]
    fn valid_naming() {
        let app = builder::kojamp_app();
        let matching_cases = [
            vec!["kojamp", "new", "Foo"],
            vec!["kojamp", "new", "Bar"],
            vec!["kojamp", "new", "DoubleWord"],
            vec!["kojamp", "new", "Number2Name"],
            vec!["kojamp", "new", "Baz"],
        ];

        for args in matching_cases {
            let matching = app.get_subcommand_matching(args);
            let project = Project::from_match(&matching);
            assert!(
                project.name_is_valid(),
                "Was expecting a valid name, but invalid was returned with `{:?}` value",
                project.name
            );
        }
    }

    #[test]
    fn invalid_naming() {
        let app = builder::kojamp_app();
        let matching_cases = [
            vec!["kojamp", "new", "lowercaseInitial"],
            vec!["kojamp", "new", "5tartingW1thNumb3r"],
            vec!["kojamp", "new", "Unallowed-Character"],
            vec!["kojamp", "new", "CharacterÁccênt"],
            vec!["kojamp", "new"],
        ];

        for args in matching_cases {
            let matching = app.get_subcommand_matching(args);
            let project = Project::from_match(&matching);
            assert!(
                !project.name_is_valid(),
                "Was expecting an invalid name, but valid one was returned with `{:?}` value",
                project.name
            );
        }
    }

    #[test]
    fn valid_project_type() {
        let app = builder::kojamp_app();
        let matching_cases = [
            vec!["kojamp", "new", "--type", "java"],
            vec!["kojamp", "new", "--type", "Java"],
            vec!["kojamp", "new", "--type", "J"],
            vec!["kojamp", "new", "--type", "j"],
            vec!["kojamp", "new", "--type", "kotlin"],
            vec!["kojamp", "new", "--type", "KoTlIn"],
            vec!["kojamp", "new", "--type", "K"],
            vec!["kojamp", "new", "--type", "k"],
        ];

        for args in matching_cases {
            let matching = app.get_subcommand_matching(args);
            let project = Project::from_match(&matching);
            let cur_type = project.get_type();
            assert!(
                project.type_is_valid(),
                "Was expecting a valid type, but invalid one was returned (`{:?}`)",
                cur_type
            );
        }
    }

    #[test]
    fn invalid_project_type() {
        let app = builder::kojamp_app();
        let matching_cases = [
            vec!["kojamp", "new", "--type", "NotJava"],
            vec!["kojamp", "new", "--type", "N"],
            vec!["kojamp", "new", "--type", "Ja-va"],
            vec!["kojamp", "new", "--type", "Gotlin"],
            vec!["kojamp", "new", "--type", "kótlin"],
        ];

        for args in matching_cases {
            let matching = app.get_subcommand_matching(args);
            let project = Project::from_match(&matching);
            let cur_type = project.get_type();
            assert!(
                !project.type_is_valid(),
                "Was expecting an invalid type, but a valid one was returned (`{:?}`)",
                cur_type
            );
        }
    }
}

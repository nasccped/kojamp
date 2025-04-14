use crate::utils::{
    io,
    strings::{StringChecker, StringTransform},
};
use clap::ArgMatches;

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectType {
    Java,
    Kotlin,
    #[allow(dead_code)]
    Undefined(String),
}

impl ProjectType {
    fn from_string(input: String) -> Self {
        match input.to_lowercase().as_str() {
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
pub struct ProjectFields {
    name: Option<String>,
    path: Option<String>,
    project_type: Option<ProjectType>,
    authors: Option<Vec<String>>,
    no_git: bool,
    prompt: bool,
}

impl ProjectFields {
    pub fn from_match(matching: &ArgMatches) -> Self {
        let name: Option<String> = matching.get_one("name").cloned();
        let path = if let Some(given_path) = matching.get_one::<String>("path").cloned() {
            Some(given_path)
        } else if let Some(name_value) = name.clone() {
            Some(StringTransform::to_kebab_case(name_value))
        } else {
            None
        };
        let project_type: Option<ProjectType> = match matching.get_one::<String>("type").cloned() {
            Some(val) => Some(ProjectType::from_string(val)),
            _ => None,
        };
        let mut authors: Option<Vec<String>> = matching
            .get_many("authors")
            .map(|vector| vector.cloned().collect());
        if authors.is_some() {
            authors = Some(
                authors
                    .unwrap()
                    .into_iter()
                    .map(|val| StringTransform::to_title_case(val))
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

        if !StringChecker::chars_in_range(&project_name, &char_range) {
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

    pub fn prompt_mode(&mut self) {
        let mut name = String::new();
        let path: String;
        let mut project_type: ProjectType;
        let mut authors: Vec<String> = Vec::new();
        let git_repo: bool;

        println!("You can use Ctrl-C at any time to abort!");

        loop {
            if self.name_is_valid() {
                break;
            }
            println!("? Project Name: \x1b[3;90m(Try using a CamelCase name with no symbols, accents or withespaces)\x1b[0m");
            name = io::read_line();

            self.name = Some(name.clone());
        }

        let kebab_name = StringTransform::to_kebab_case(name);

        println!(
            "? Project Path: \x1b[3;90m(Press Enter to use name in kebab case `{}`)\x1b[0m",
            kebab_name
        );

        path = match (io::read_line(), kebab_name) {
            (x, name) if !x.is_empty() => name,
            (_, name) => name,
        };

        self.path = Some(path);

        loop {
            if self.type_is_valid() {
                break;
            }
            println!(
                "? Project Type: \x1b[3;90m(\x1b[92m[J]\x1b[90mava or \x1b[92m[K]\x1b[90motlin)\x1b[0m"
            );

            project_type = ProjectType::from_string(io::read_line());
            self.project_type = Some(project_type);
        }

        let mut author_buf: String;
        let stop_case = String::from("!CONTINUE");

        loop {
            println!(
                "? Project Authors (Type \x1b[92m{}\x1b[0m to quit): \x1b[3;90m[{}]\x1b[0m",
                stop_case,
                authors.join(", ")
            );
            author_buf = io::read_line();

            if author_buf == stop_case {
                break;
            }

            println!(
                "no trans: {}\nwt trans: {}",
                author_buf,
                StringTransform::to_title_case(author_buf.clone())
            );
            authors.push(StringTransform::to_title_case(author_buf));
        }

        self.authors = Some(authors);

        println!("? Did you want to initialize a git repo? \x1b[3;90m(Just press Enter (or yes/y) if so, else, type anything)\x1b[0m");
        git_repo = match io::read_line() {
            x if x.is_empty() => true,
            x if ["yes", "y"].contains(&x.to_lowercase().as_str()) => true,
            _ => false,
        };
        self.no_git = !git_repo;
    }
}

#[cfg(test)]
mod project_naming {

    use super::*;
    use crate::{kojamp::builder, utils::arg_test::ARG_BUILDER};

    #[test]
    fn valid_naming() {
        let app = builder::kojamp_app();
        let matching_cases = [
            ["new", "Foo"],
            ["new", "Bar"],
            ["new", "DoubleWord"],
            ["new", "Number2Name"],
            ["new", "Baz"],
        ];

        for case in matching_cases {
            let args = ARG_BUILDER.args_from(case);
            let matching = app.get_subcommand_matching(args);
            let project = ProjectFields::from_match(&matching);
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
            ["new", "lowercaseInitial"],
            ["new", "5tartingW1thNumb3r"],
            ["new", "Unallowed-Character"],
            ["new", "CharacterÁccênt"],
            ["new", ""],
        ];

        for case in matching_cases {
            let args = ARG_BUILDER.args_from(case);
            let matching = app.get_subcommand_matching(args);
            let project = ProjectFields::from_match(&matching);
            assert!(
                !project.name_is_valid(),
                "Was expecting an invalid name, but valid one was returned with `{:?}` value",
                project.name
            );
        }
    }
}

#[cfg(test)]
mod projectfields_type_validation {

    use super::*;
    use crate::{kojamp::builder, utils::arg_test::ARG_BUILDER};

    #[test]
    fn valid_project_type() {
        let app = builder::kojamp_app();
        let matching_cases = [
            ["new", "--type", "java"],
            ["new", "--type", "Java"],
            ["new", "--type", "J"],
            ["new", "--type", "j"],
            ["new", "--type", "kotlin"],
            ["new", "--type", "KoTlIn"],
            ["new", "--type", "K"],
            ["new", "--type", "k"],
        ];

        for case in matching_cases {
            let args = ARG_BUILDER.args_from(case);
            let matching = app.get_subcommand_matching(args);
            let project = ProjectFields::from_match(&matching);
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
            ["new", "--type", "NotJava"],
            ["new", "--type", "N"],
            ["new", "--type", "Ja-va"],
            ["new", "--type", "Gotlin"],
            ["new", "--type", "kótlin"],
        ];

        for case in matching_cases {
            let args = ARG_BUILDER.args_from(case);
            let matching = app.get_subcommand_matching(args);
            let project = ProjectFields::from_match(&matching);
            let cur_type = project.get_type();
            assert!(
                !project.type_is_valid(),
                "Was expecting an invalid type, but a valid one was returned (`{:?}`)",
                cur_type
            );
        }
    }
}

#[cfg(test)]
mod projecttype_asserting {

    use super::ProjectType;

    #[test]
    fn expecting_java() {
        let types = ["J", "j", "java", "JaVa"].iter().map(|val| val.to_string());

        for t in types {
            let p_type = ProjectType::from_string(t);
            assert_eq!(
                p_type,
                ProjectType::Java,
                "A strange type (`{:?}`) has been returned when Java type was expected",
                p_type
            );
        }
    }

    #[test]
    fn expecting_kotlin() {
        let types = ["K", "k", "kotlin", "kOtLiN"]
            .iter()
            .map(|val| val.to_string());

        for t in types {
            let p_type = ProjectType::from_string(t);
            assert_eq!(
                p_type,
                ProjectType::Kotlin,
                "A strange type (`{:?}`) has been returned when Java type was expected",
                p_type
            );
        }
    }

    #[test]
    fn expecting_undefined() {
        let types = ["Undefined", "a", "What", "TypeISTHIs"]
            .iter()
            .map(|val| val.to_string());

        for t in types {
            let p_type = ProjectType::from_string(t.clone());
            assert_eq!(
                p_type,
                ProjectType::Undefined(t),
                "A strange type (`{:?}`) has been returned when Java type was expected",
                p_type
            );
        }
    }
}

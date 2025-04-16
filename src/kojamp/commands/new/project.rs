use clap::ArgMatches;
use std::fmt;

use crate::utils::{
    io,
    strings::{StringChecker, StringTransform},
};

#[derive(Clone)]
pub struct ProjectName(Option<String>);

impl ProjectName {
    pub fn new(matches: &ArgMatches) -> Self {
        Self(matches.get_one::<String>("name").cloned())
    }

    pub fn set<T: Into<String>>(&mut self, new_value: T) {
        let new_value = new_value.into();
        self.0 = Some(new_value);
    }

    pub fn is_valid(&self) -> bool {
        let project_name = self.0.clone();

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
}

impl fmt::Display for ProjectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self.0 {
                Some(value) => format!("{}{}{}", "\x1b[92m", value, "\x1b[0m"),
                _ => format!("{}{}{}", "\x1b[91m", "UNDEFINED", "\x1b[0m"),
            }
        )
    }
}

#[derive(Clone)]
pub struct ProjectPath(Option<String>);

impl ProjectPath {
    pub fn new(matches: &ArgMatches) -> Self {
        Self(matches.get_one::<String>("path").cloned())
    }

    pub fn update_to_name_as_kebab(&mut self, project_name: &ProjectName) {
        if let Some(name) = project_name.0.clone() {
            self.0 = Some(StringTransform::to_kebab_case(name))
        }
    }

    pub fn set<T: Into<String>>(&mut self, new_value: T) {
        let new_value = new_value.into();
        self.0 = Some(new_value);
    }
}

impl fmt::Display for ProjectPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self.0 {
                Some(value) => format!("{}{}{}", "\x1b[92m", value, "\x1b[0m"),
                _ => format!("{}{}{}", "\x1b[91m", "UNDEFINED", "\x1b[0m"),
            }
        )
    }
}

pub enum ProjectType {
    Java,
    Kotlin,
    Undefined(String),
    None,
}

impl ProjectType {
    pub fn new(matches: &ArgMatches) -> Self {
        if let Some(p_type) = matches.get_one::<String>("type").cloned() {
            match p_type.to_lowercase().as_str() {
                "j" | "java" => Self::Java,
                "k" | "kotlin" => Self::Kotlin,
                _ => Self::Undefined(p_type),
            }
        } else {
            Self::None
        }
    }

    pub fn set<T: Into<String>>(&mut self, new_value: T) {
        let new_value = new_value.into();

        match new_value.to_lowercase().as_str() {
            "j" | "java" => *self = Self::Java,
            "k" | "kotlin" => *self = Self::Kotlin,
            _ => *self = Self::Undefined(new_value),
        }
    }

    pub fn is_valid(&self) -> bool {
        match self {
            ProjectType::Java | ProjectType::Kotlin => true,
            _ => false,
        }
    }
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            match &self {
                ProjectType::None | ProjectType::Undefined(_) => "\x1b[91m",
                _ => "\x1b[92m",
            },
            match &self {
                ProjectType::Java => format!("{}{}{}", "\x1b[92m", "Java", "\x1b[0m"),
                ProjectType::Kotlin => format!("{}{}{}", "\x1b[92m", "Kotlin", "\x1b[0m"),
                ProjectType::Undefined(s) =>
                    format!("{}[UNDEFINED {}]{}", "\x1b[92m", s, "\x1b[0m"),
                ProjectType::None => format!("{}{}{}", "\x1b[92m", "[UNDEFINED - NONE]", "\x1b[0m"),
            },
            "\x1b[0m"
        )
    }
}

#[derive(Clone)]
pub struct ProjectAuthors(Option<Vec<String>>);

impl ProjectAuthors {
    pub fn new(matches: &ArgMatches) -> Self {
        if let Some(authors) = matches
            .get_many("authors")
            .map(|vector| vector.cloned().collect::<Vec<String>>())
        {
            Self(Some(
                authors
                    .into_iter()
                    .map(|val| StringTransform::to_title_case(val))
                    .collect::<Vec<String>>()
                    .join(" ")
                    .split("/")
                    .filter(|name| !name.is_empty())
                    .map(|name| name.trim().to_string())
                    .collect(),
            ))
        } else {
            Self(None)
        }
    }

    pub fn push<T: Into<String> + Clone>(&mut self, value: T) {
        if self.0.is_none() {
            self.0 = Some(Vec::new());
        }

        if value
            .clone()
            .into()
            .chars()
            .filter(|&c| c == '/' || c == '!' || c == '?')
            .count()
            > 0
        {
            return;
        }

        let value: String = value
            .into()
            .split(" ")
            .map(|word| StringTransform::to_title_case(word))
            .collect::<Vec<String>>()
            .join(" ");

        if let Some(vector) = &mut self.0 {
            vector.push(value);
        }
    }

    pub fn clear(&mut self) {
        self.0 = None;
    }
}

impl fmt::Display for ProjectAuthors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            "\x1b[92m",
            match &self.0 {
                None => "[]".to_string(),
                Some(vector) => vector.join(", "),
            },
            "\x1b[0m"
        )
    }
}

pub struct GitRepository(bool);

impl GitRepository {
    pub fn new(matches: &ArgMatches) -> Self {
        Self(!matches.get_flag("no-git"))
    }

    pub fn set(&mut self, value: bool) {
        self.0 = value;
    }
}

impl fmt::Display for GitRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            if self.0 {
                "\x1b[92m".to_string() + "Yes"
            } else {
                "\x1b[91m".to_string() + "No"
            },
            "\x1b[0m"
        )
    }
}

pub struct ProjectComposition {
    name: ProjectName,
    path: ProjectPath,
    project_type: ProjectType,
    project_authors: ProjectAuthors,
    git_repo: GitRepository,
}

impl ProjectComposition {
    pub fn new_from_matches(matches: &ArgMatches) -> Self {
        Self {
            name: ProjectName::new(matches),
            path: ProjectPath::new(matches),
            project_type: ProjectType::new(matches),
            project_authors: ProjectAuthors::new(matches),
            git_repo: GitRepository::new(matches),
        }
    }

    pub fn new_from_prompt_mode(matches: &ArgMatches) -> Self {
        let mut project_composition = Self::new_from_matches(matches);

        let mut name = project_composition.name.clone();
        let mut path = project_composition.path.clone();
        let mut p_type = project_composition.project_type;
        let mut authors = project_composition.project_authors;
        let mut git_repo = project_composition.git_repo;

        loop {
            if name.is_valid() {
                println!(
                    "{0}◆{1} Project Name: {2}{3}{4}",
                    "\x1b[92m", // 0
                    "\x1b[0m",  // 1
                    "\x1b[92m", // 2
                    name,       // 3
                    "\x1b[0m",  // 4
                );
                break;
            }

            println!(
                "{0}◆{1} Project Name: {2}(Try using a CamelCase name with no symbols, accents or withespaces){3}",
                "\x1b[91m",   // 0
                "\x1b[0m",    // 1
                "\x1b[3;90m", // 2
                "\x1b[0m"     // 3
            );

            name.set(io::input("> "));
            io::remove_lines(2);
        }

        let kebab_name = StringTransform::to_kebab_case(name.clone().0.unwrap());

        println!(
            "{0}◆{1} Project Path: {2}(Press Enter to use name in kebab case `{3}`){4}",
            "\x1b[91m",   // 0
            "\x1b[0m",    // 1
            "\x1b[3;90m", // 2
            kebab_name,   // 3
            "\x1b[0m"     // 4
        );

        let new_path = io::input("> ");

        if new_path.is_empty() {
            path.set(kebab_name);
        } else {
            path.set(new_path);
        }

        io::remove_lines(2);

        println!(
            "{0}◆{1} Project Path: {2}",
            "\x1b[92m", // 0
            "\x1b[0m",  // 1
            path,       // 3
        );

        loop {
            if p_type.is_valid() {
                break;
            }

            println!(
                "{0}◆{1} Project Type: {2}({3}[J]{4}ava or {5}[K]{6}otlin){7}",
                "\x1b[91m",   // 0
                "\x1b[0m",    // 1
                "\x1b[3;90m", // 2
                "\x1b[92m",   // 3
                "\x1b[90m",   // 4
                "\x1b[92m",   // 5
                "\x1b[90m",   // 6
                "\x1b[0m"     // 7
            );

            p_type.set(io::input("> "));
            io::remove_lines(2);
        }

        println!(
            "{0}◆{1} Project Type: {2}",
            "\x1b[92m", // 0
            "\x1b[0m",  // 1
            p_type
        );

        let mut cur_author: String;

        loop {
            println!(
                "{0}◆{1} Project Authors (Type {2}!CONTINUE{3} when done or {4}!NONE{5} to abort): {6}{7}{8}",
                "\x1b[91m",
                "\x1b[0m",
                "\x1b[92m",
                "\x1b[0m",
                "\x1b[91m",
                "\x1b[0m",
                "\x1b[3;90m",
                if let Some(aut) = &authors.0 {
                    "[".to_string() + &aut.join(", ") + "]"
                } else {
                    "[]".to_string()
                },
                "\x1b[0m",
            );

            cur_author = io::input("> ");

            match cur_author.as_str() {
                "!CONTINUE" => break,
                "!NONE" => {
                    authors.clear();
                    break;
                }
                x => authors.push(x),
            }
            io::remove_lines(2);
        }

        io::remove_lines(2);

        println!(
            "{0}◆{1} Project Authors: {2}",
            "\x1b[92m", "\x1b[0m", authors
        );

        let mut git_init: String;

        loop {
            println!(
                "{0}◆{1} Did you want to initialize a git repo? {2}({3}[Y]{4}es | {5}[N]{6}o){7}",
                "\x1b[91m",
                "\x1b[0m",
                "\x1b[3;90m",
                "\x1b[92m",
                "\x1b[90m",
                "\x1b[92m",
                "\x1b[90m",
                "\x1b[0m"
            );

            git_init = io::input("> ");
            io::remove_lines(2);

            match git_init.to_lowercase().as_str() {
                "y" | "yes" => git_repo.set(true),
                "n" | "no" => git_repo.set(false),
                _ => continue,
            }

            break;
        }

        println!(
            "{0}◆{1} Did you want to initialize a git repo? {2}",
            "\x1b[92m", "\x1b[0m", git_repo
        );

        project_composition.name = name;
        project_composition.path = path;
        project_composition.project_type = p_type;
        project_composition.project_authors = authors;
        project_composition.git_repo = git_repo;

        project_composition
    }
}

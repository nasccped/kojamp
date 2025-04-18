use clap::ArgMatches;
use std::{borrow::Cow, fmt};

use crate::{
    utils::{
        io::{self, IOAsking, IOReporting, ReportStatus},
        strings::{StringChecker, StringTransform},
    },
    vec_dispbox,
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

impl From<&ProjectName> for Cow<'static, str> {
    fn from(s: &ProjectName) -> Self {
        Cow::Owned(format!(
            "{}{}{}",
            if s.0.is_some() {
                "\x1b[92m"
            } else {
                "\x1b[91m"
            },
            s.0.as_ref().unwrap_or(&"[UNDEFINED]".to_string()),
            "\x1b[0m"
        ))
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

impl From<&ProjectPath> for Cow<'static, str> {
    fn from(s: &ProjectPath) -> Self {
        Cow::Owned(format!(
            "{}{}{}",
            if s.0.is_some() {
                "\x1b[92m"
            } else {
                "\x1b[91m"
            },
            s.0.as_ref().unwrap_or(&"[UNDEFINED]".to_string()),
            "\x1b[0m"
        ))
    }
}

#[derive(Debug, PartialEq)]
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

impl From<&ProjectType> for Cow<'static, str> {
    fn from(s: &ProjectType) -> Self {
        Cow::Owned(format!(
            "{}{}{}",
            match s {
                ProjectType::None | ProjectType::Undefined(_) => "\x1b[91m",
                _ => "\x1b[92m",
            },
            match s {
                ProjectType::Java => format!("{}{}{}", "\x1b[92m", "Java", "\x1b[0m"),
                ProjectType::Kotlin => format!("{}{}{}", "\x1b[92m", "Kotlin", "\x1b[0m"),
                ProjectType::Undefined(s) =>
                    format!("{}[UNDEFINED - {}]{}", "\x1b[91m", s, "\x1b[0m"),
                ProjectType::None => format!("{}{}{}", "\x1b[91m", "[UNDEFINED - NONE]", "\x1b[0m"),
            },
            "\x1b[0m"
        ))
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
            .filter(|c| ['?', '/', '!'].contains(c))
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

    pub fn is_valid(&self) -> bool {
        if self.0.is_none() {
            return true;
        }

        self.0.as_ref().unwrap().iter().all(|aut| {
            aut.chars()
                .filter(|c| ['?', '/', '!', ','].contains(c))
                .count()
                == 0
        })
    }
}

impl From<&ProjectAuthors> for Cow<'static, str> {
    fn from(s: &ProjectAuthors) -> Self {
        Cow::Owned(format!(
            "{}{}",
            match s.0.as_ref() {
                None => "\x1b[91mNONE".to_string(),
                Some(vector) => format!("\x1b[92m[{}]", vector.join(", ")),
            },
            "\x1b[0m"
        ))
    }
}

impl fmt::Display for ProjectAuthors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            match self.0.as_ref() {
                Some(authors) => authors.join(", "),
                None => "".into(),
            }
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

trait StrFromBool {
    fn from_bool(b: bool) -> Self;
}

impl StrFromBool for Cow<'static, str> {
    fn from_bool(b: bool) -> Self {
        Cow::Owned(format!(
            "{}{}",
            if b { "\x1b[92mYes" } else { "\x1b[91mNo" },
            "\x1b[0m"
        ))
    }
}

impl From<&GitRepository> for Cow<'static, str> {
    fn from(s: &GitRepository) -> Self {
        Cow::Owned(format!(
            "{}{}",
            if s.0 { "\x1b[92mYes" } else { "\x1b[91mNo" },
            "\x1b[0m"
        ))
    }
}

pub struct ProjectComposition {
    name: ProjectName,
    path: ProjectPath,
    project_type: ProjectType,
    project_authors: ProjectAuthors,
    git_repo: bool,
}

impl ProjectComposition {
    pub fn new_from_matches(matches: &ArgMatches) -> Self {
        Self {
            name: ProjectName::new(matches),
            path: ProjectPath::new(matches),
            project_type: ProjectType::new(matches),
            project_authors: ProjectAuthors::new(matches),
            git_repo: !matches.get_flag("no-git"),
        }
    }

    pub fn new_from_prompt_mode(matches: &ArgMatches) -> Self {
        let mut project_composition = Self::new_from_matches(matches);

        let mut name = project_composition.name.clone();
        let mut path = project_composition.path.clone();
        let mut p_type = project_composition.project_type;
        let mut authors = project_composition.project_authors;

        let question_ind = "+";
        let input_ind = "- ";
        let response_escape = "\x1b[3;96m";

        let io_report: IOReporting = IOReporting::new(
            ReportStatus::Warn,
            Some("Prompt mode alerts"),
            vec_dispbox![
                format!(
                    "We'll start an {}Ask-Answer{} scheme to set valuable data",
                    "\x1b[93m", "\x1b[0m"
                ),
                "about your project. Don't worry if you accidentally give",
                format!(
                    "{}invalid{} responses. The program will ask you again {}:^D{}",
                    "\x1b[91m", "\x1b[0m", "\x1b[3;92m", "\x1b[0m"
                ),
                "",
                format!(
                    "You can {}abort{} at any time by using the {}<Ctrl-C>{} combination!",
                    "\x1b[91m", "\x1b[0m", "\x1b[96m", "\x1b[0m"
                )
            ],
        );

        io_report.print_content();

        let (mut asker, mut response): (IOAsking, String);

        let binding =
            "\x1b[3;90m(Try using a CamelCase name with no symbols, accents or withespaces)\x1b[0m";

        asker = IOAsking::new(
            question_ind,
            "Project Name:",
            binding,
            input_ind,
            response_escape,
        );

        loop {
            if name.is_valid() {
                asker.set_answered(true);
                asker.update_hint(&name);
                asker.print_content();
                break;
            }
            response = asker.ask();
            name.set(response);
            io::remove_lines(2);
        }

        let kebab_name = StringTransform::to_kebab_case(name.clone().0.unwrap());

        let binding = "\x1b[3;90m(Just press Enter to use name in kebab case)\x1b[0m";

        asker = IOAsking::new(
            question_ind,
            "Project Path:",
            binding,
            input_ind,
            response_escape,
        );

        response = asker.ask();

        if response.is_empty() {
            path.set(kebab_name);
        } else {
            path.set(response);
        }

        io::remove_lines(2);
        asker.set_answered(true);
        asker.update_hint(&path);
        asker.print_content();

        let binding = "\x1b[3;90m(\x1b[92m[J]\x1b[90mava or \x1b[92m[K]\x1b[90motlin)\x1b[0m";

        asker = IOAsking::new(
            question_ind,
            "Project Type:",
            binding,
            input_ind,
            response_escape,
        );

        loop {
            if p_type.is_valid() {
                asker.set_answered(true);
                asker.update_hint(&p_type);
                asker.print_content();
                break;
            }

            response = asker.ask();

            p_type.set(response);
            io::remove_lines(2);
        }

        let binding = "Project Authors (Type \x1b[92m!CONTINUE\x1b[0m when done or \x1b[91m!NONE\x1b[0m to no author / avoid using '/', '?' and '!' chars):";
        asker = IOAsking::new(question_ind, binding, "", input_ind, response_escape);

        loop {
            asker.update_hint(format!("{}{}{}", "\x1b[3;90m", authors, "\x1b[0m"));

            response = asker.ask();

            match response.as_str() {
                x if ["!CONTINUE", "!NONE"].contains(&x) => {
                    if x == "!NONE" {
                        authors.clear();
                    }
                    io::remove_lines(1);
                    asker.update_question("Project Authors:");
                    asker.update_hint(&authors);
                    asker.set_answered(true);
                    asker.print_content();
                    break;
                }
                x => authors.push(x),
            }
            io::remove_lines(2);
        }

        io::remove_lines(2);
        asker.update_hint(&authors);
        asker.set_answered(true);
        asker.print_content();

        let binding = "\x1b[3;90m(\x1b[92m[Y]\x1b[90mes or \x1b[92m[N]\x1b[90mo)\x1b[0m";

        asker = IOAsking::new(
            question_ind,
            "Did you want to initialize a git repo?",
            binding.as_ref(),
            input_ind,
            response_escape,
        );

        let git_repo: bool;

        loop {
            response = asker.ask();
            io::remove_lines(2);

            match response.to_lowercase().as_str() {
                x if ["y", "yes", "n", "no"].contains(&x) => {
                    match x {
                        "y" | "yes" => git_repo = true,
                        _ => git_repo = false,
                    }

                    asker.set_answered(true);
                    asker.update_hint(Cow::from_bool(git_repo));
                    asker.print_content();
                }
                _ => continue,
            }

            break;
        }

        project_composition.name = name;
        project_composition.path = path;
        project_composition.project_type = p_type;
        project_composition.project_authors = authors;
        project_composition.git_repo = git_repo;

        project_composition
    }

    pub fn destructure(
        &self,
    ) -> (
        &ProjectName,
        &ProjectPath,
        &ProjectType,
        &ProjectAuthors,
        bool,
    ) {
        (
            &self.name,
            &self.path,
            &self.project_type,
            &self.project_authors,
            self.git_repo,
        )
    }
}

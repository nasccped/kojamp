use crate::{
    globals::{FAILURE_EXIT_STATUS, SUCCESS_EXIT_STATUS},
    utils::{report, string::StringTransformation},
};
use clap::ArgMatches;
use colored::Colorize;
use std::{convert::TryFrom, env, fmt, path::PathBuf, rc::Rc};

struct ProjectName(Rc<str>);

impl From<&ArgMatches> for ProjectName {
    fn from(value: &ArgMatches) -> Self {
        Self(Rc::from(
            value
                .get_one::<String>("name")
                .cloned()
                // NOTE: We can unwrap the name 'cause it's a required
                // argument, so will never fail
                .unwrap()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" "),
        ))
    }
}

enum ProjectKind {
    Java,
    Kotlin,
    Undefined(Rc<str>),
}

impl From<&ArgMatches> for ProjectKind {
    fn from(value: &ArgMatches) -> Self {
        let kind = match value.get_one::<String>("kind") {
            Some(x) => x,
            None => return Self::Undefined(Rc::from("")),
        };

        match kind.to_lowercase().trim().as_ref() {
            "java" => Self::Java,
            "kotlin" => Self::Kotlin,
            _ => Self::Undefined(Rc::from(kind.as_ref())),
        }
    }
}

impl fmt::Display for ProjectKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProjectKind::Java => "Java".bright_blue().bold(),
                ProjectKind::Kotlin => "Kotlin".bright_blue().bold(),
                ProjectKind::Undefined(x) => format!("Undefined({})", x).bright_red().bold(),
            }
        )
    }
}

struct ProjectPath(PathBuf);

impl TryFrom<&ArgMatches> for ProjectPath {
    type Error = ();

    fn try_from(value: &ArgMatches) -> Result<Self, Self::Error> {
        match (env::current_dir(), value.get_one::<String>("path")) {
            (Ok(cur), Some(path)) => Ok(Self(cur.join(path))),
            _ => Err(()),
        }
    }
}

impl TryFrom<&ProjectName> for ProjectPath {
    type Error = ();

    fn try_from(value: &ProjectName) -> Result<Self, Self::Error> {
        match env::current_dir() {
            Err(_) => Err(()),
            Ok(cur) => Ok(Self(cur.join(value.0.as_ref().to_kebab_case()))),
        }
    }
}

impl From<PathBuf> for ProjectPath {
    fn from(value: PathBuf) -> Self {
        Self(value)
    }
}

struct ProjectAuthors(Option<Vec<String>>);

impl From<&ArgMatches> for ProjectAuthors {
    fn from(value: &ArgMatches) -> Self {
        let authors = value.get_one::<String>("authors").map(|aut| {
            aut.split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
                .split(",")
                .map(|name| name.trim().to_string())
                .collect()
        });
        Self(authors)
    }
}

struct ProjectFields {
    project_name: ProjectName,
    project_kind: ProjectKind,
    project_authors: ProjectAuthors,
    project_repo: bool,
}

struct ProjectFieldsConstructor {
    project_name: Option<ProjectName>,
    project_kind: Option<ProjectKind>,
    project_authors: Option<ProjectAuthors>,
    project_repo: Option<bool>,
}

impl ProjectFieldsConstructor {
    fn new() -> Self {
        Self {
            project_name: None,
            project_kind: None,
            project_authors: None,
            project_repo: None,
        }
    }

    fn name(mut self, name: ProjectName) -> Self {
        self.project_name = Some(name);
        self
    }

    fn kind(mut self, kind: ProjectKind) -> Self {
        self.project_kind = Some(kind);
        self
    }

    fn authors(mut self, authors: ProjectAuthors) -> Self {
        self.project_authors = Some(authors);
        self
    }

    fn repo(mut self, repo: bool) -> Self {
        self.project_repo = Some(repo);
        self
    }

    fn build(self) -> ProjectFields {
        let project_name = self.project_name.expect(
            "Couldn't take ProjectName. You probably missed the ProjectFieldsConstructor.name function",
        );
        let project_kind = self.project_kind.expect(
            "Couldn't take ProjectKind. You probably missed the ProjectFieldsConstructor.kind function",
        );
        let project_authors = self.project_authors.expect(
            "Couldn't take ProjectAuthors. You probably missed the ProjectFieldsConstructor.authors function",
        );
        let project_repo = self.project_repo.expect(
            "Couldn't take bool for project_repo. You probably missed the ProjectFieldsConstructor.repo function",
        );

        ProjectFields {
            project_name,
            project_kind,
            project_authors,
            project_repo,
        }
    }
}

fn from_new(fields: ProjectFields, matching: &ArgMatches) -> i32 {
    let path = if let Ok(p) = ProjectPath::try_from(matching) {
        p
    } else if let Ok(p) = ProjectPath::try_from(&fields.project_name) {
        p
    } else {
        report::path::undefined_cur_dir();
        return FAILURE_EXIT_STATUS;
    };

    println!(
        "Creating a new `{}` project ({}) on a new",
        fields.project_kind,
        fields.project_name.0.bright_green()
    );
    println!("directory: `{}`", path.0.to_str().unwrap().bright_yellow());
    SUCCESS_EXIT_STATUS
}

fn from_init(fields: ProjectFields) -> i32 {
    let cur_path = if let Ok(p) = env::current_dir() {
        p
    } else {
        report::path::undefined_cur_dir();
        return FAILURE_EXIT_STATUS;
    };

    let path = ProjectPath::from(cur_path);

    println!(
        "Creating a new `{}` project ({}) on the current",
        fields.project_kind,
        fields.project_name.0.bright_green()
    );
    println!("directory: `{}`", path.0.to_string_lossy().bright_yellow());
    SUCCESS_EXIT_STATUS
}

pub fn main(pair: (&str, ArgMatches)) -> i32 {
    let (cmd, matching) = (pair.0, &pair.1);
    let name = ProjectName::from(matching);
    let kind = ProjectKind::from(matching);
    let authors = ProjectAuthors::from(matching);
    let git_repo = !matching.get_flag("no-git");

    let project_fields: ProjectFields = ProjectFieldsConstructor::new()
        .name(name)
        .kind(kind)
        .authors(authors)
        .repo(git_repo)
        .build();

    match (cmd, matching) {
        ("new", matching) => from_new(project_fields, matching),
        _ => from_init(project_fields),
    }
}

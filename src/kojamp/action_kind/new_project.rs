use crate::utils::{report, string::StringTransformation};
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
            (Err(_), _) => Err(()),
            (_, None) => Err(()),
            (Ok(cur), Some(path)) => Ok(Self(cur.join(path))),
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

pub fn main(pair: (&str, ArgMatches)) -> i32 {
    let cur_path = match env::current_dir() {
        Err(_) => {
            report::path::undefined_cur_dir();
            return 1;
        }
        Ok(p) => p,
    };

    match pair {
        ("new", matching) => from_new(matching),
        (_, matching) => from_init(matching, cur_path),
    }
}

fn from_new(matching: ArgMatches) -> i32 {
    let name = ProjectName::from(&matching);
    let path = if let Ok(p) = ProjectPath::try_from(&matching) {
        p
    } else if let Ok(p) = ProjectPath::try_from(&name) {
        p
    } else {
        report::path::undefined_cur_dir();
        return 1;
    };
    let kind = ProjectKind::from(&matching);

    println!(
        "Creating a new `{}` project ({}) on a new",
        kind,
        name.0.bright_green()
    );
    println!("directory: `{}`", path.0.to_str().unwrap().bright_yellow());
    0
}

fn from_init(matching: ArgMatches, cur_path: PathBuf) -> i32 {
    let name = ProjectName::from(&matching);
    let path = ProjectPath::from(cur_path);
    let kind = ProjectKind::from(&matching);

    println!(
        "Creating a new `{}` project ({}) on the current",
        kind,
        name.0.bright_green()
    );
    println!("directory: `{}`", path.0.to_str().unwrap().bright_yellow());
    0
}

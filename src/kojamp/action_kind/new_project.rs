use crate::{
    globals::{FAILURE_EXIT_STATUS, PROGRAM_REPO_URL, SUCCESS_EXIT_STATUS},
    utils::{report, string::StringTransformation},
};
use clap::ArgMatches;
use colored::Colorize;
use serde::Serialize;
use std::{
    convert::TryFrom,
    env, fmt, fs,
    path::{Path, PathBuf},
    rc::Rc,
};

struct ProjectName(Rc<str>);

impl ProjectName {
    /// Test ProjectName validity for ProjectName value
    fn is_valid(&self) -> bool {
        // if empty str
        if self.0.is_empty() {
            return false;
        }

        // get chars
        let mut self_chars = self.0.as_ref().chars();

        // NOTE: unwrap is safe here 'cause we have already checked if inner value is empty
        let next = self_chars.next().unwrap();

        // the first char should be uppercase
        if !('A'..='Z').contains(&next) {
            return false;
        }

        // consume the created chars + check if all in range of acceptable java file name
        self_chars.into_iter().all(|c| c.is_digit(36))
    }
}

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

impl ProjectKind {
    fn is_valid(&self) -> bool {
        matches!(self, ProjectKind::Java | ProjectKind::Kotlin)
    }
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

impl ProjectPath {
    fn is_valid(&self, should_exists: bool) -> bool {
        let path = &self.0;

        if path.file_name().is_none() {
            return false;
        }

        match path.parent() {
            None => return false,
            Some(x) if x == Path::new("") => return false,
            Some(x) if !x.exists() => return false,
            _ => {}
        }

        path.exists() == should_exists
    }
}

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

#[derive(Serialize)]
struct MainToml<'a> {
    #[serde(rename = "project")]
    project: ProjectToml<'a>,
}

#[derive(Serialize)]
struct ProjectToml<'a> {
    name: Rc<str>,
    kind: &'a str,
    authors: Rc<&'a Option<Vec<String>>>,
}

impl<'a> From<&'a ProjectFields> for ProjectToml<'a> {
    fn from(value: &'a ProjectFields) -> Self {
        Self {
            name: Rc::clone(&value.project_name.0),
            kind: match value.project_kind {
                ProjectKind::Java => "java",
                _ => "kotlin",
            },
            authors: Rc::from(&value.project_authors.0),
        }
    }
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

fn create_java_file_content(class_name: &str) -> String {
    [
        "/**",
        "* This file was generated using kojamp CLI-app",
        format!(
            "* Take a look at the official repository at {}",
            PROGRAM_REPO_URL
        )
        .as_str(),
        "*/",
        "",
        format!("public class {} {{", class_name).as_str(),
        "",
        "    private static String turnGreen(String text) {",
        "        return \"\\u001b[92m\" + text + \"\\u001b[0m\";",
        "    }",
        "",
        "    private static void println(Object o) {",
        "        System.out.println(o);",
        "    }",
        "",
        "    private static void println() {",
        "        System.out.println();",
        "    }",
        "",
        "    private static void print(Object o) {",
        "        System.out.print(o);",
        "    }",
        "",
        "    public String greeting() {",
        format!(
            "        return \"This is a hello from \" + turnGreen(\"'{}'\") + \" project!\";",
            class_name
        )
        .as_str(),
        "    }",
        "",
        "    public static void main(String[] args) {",
        "        print(\"Hi\");",
        "        print(\" \");",
        "        println(\"there\");",
        "",
        format!("        println(new {}().greeting());", class_name).as_str(),
        "    }",
        "}",
    ]
    .map(|v| v.to_string())
    .into_iter()
    .collect::<Vec<_>>()
    .join("\n")
}

fn create_kotlin_file_content(class_name: &str) -> String {
    [
        "/**",
        " * This file was generated using kojamp CLI-app,",
        format!(
            " * Take a look at the official repository at {}",
            PROGRAM_REPO_URL
        )
        .as_str(),
        " */",
        "",
        "fun turnGreen(value: String): String {",
        "    return \"\\u001b[92m$value\\u001b[0m\"",
        "}",
        "",
        "fun greeting(): String {",
        format!(
            "     return \"Hello from ${{turnGreen(\"'{}'\")}} project\"",
            class_name
        )
        .as_str(),
        "}",
        "",
        "fun main() {",
        "    print(\"Hi\")",
        "    print(\" \")",
        "    println(\"there\")",
        "",
        "    println(greeting())",
        "}",
    ]
    .map(|v| v.to_string())
    .into_iter()
    .collect::<Vec<_>>()
    .join("\n")
}

fn from_new(fields: ProjectFields, matching: &ArgMatches) -> i32 {
    let path: Option<ProjectPath> = [
        ProjectPath::try_from(matching),
        ProjectPath::try_from(&fields.project_name),
    ]
    .into_iter()
    .find(|p| p.is_ok())
    .and_then(|p| Some(p.unwrap()));

    let path = if let Some(p) = path {
        p
    } else {
        report::path::undefined_cur_dir();
        return FAILURE_EXIT_STATUS;
    };

    if !path.is_valid(false) {
        report::path::invalid_path_when_new(path.0.to_str());
        return FAILURE_EXIT_STATUS;
    }

    build_from_fields(fields, Some(path.0))
}

fn from_init(fields: ProjectFields) -> i32 {
    let path = if let Ok(p) = env::current_dir() {
        ProjectPath::from(p)
    } else {
        report::path::undefined_cur_dir();
        return FAILURE_EXIT_STATUS;
    };

    if !path.is_valid(true) {
        report::path::invalid_path_when_init(path.0.to_str());
        return FAILURE_EXIT_STATUS;
    }

    build_from_fields(fields, None)
}

fn build_from_fields(fields: ProjectFields, path: Option<PathBuf>) -> i32 {
    let mut path_handler = if let Some(p) = path {
        p
    } else {
        PathBuf::new()
    };
    path_handler.push("src");

    if fs::create_dir_all(&path_handler).is_err() {
        report::project::couldnt_create_src_dir();
        return FAILURE_EXIT_STATUS;
    }

    let project_name_str: &str = fields.project_name.0.as_ref();
    let file_content: String;
    let file_name: String;

    match fields.project_kind {
        ProjectKind::Java => {
            file_content = create_java_file_content(project_name_str);
            file_name = format!("{}.java", project_name_str);
        }
        _ => {
            file_content = create_kotlin_file_content(project_name_str);
            file_name = format!("{}.kt", project_name_str);
        }
    }

    path_handler.push(file_name);
    if fs::write(&path_handler, file_content).is_err() {
        // TODO: report error for main file building
        return FAILURE_EXIT_STATUS;
    }

    path_handler.pop();
    path_handler.pop();
    path_handler.push("Kojamp.toml");

    let t = MainToml {
        project: ProjectToml::from(&fields),
    };

    let toml_string = toml::to_string(&t)
        .unwrap()
        .lines()
        .filter(|row| !row.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    if fs::write(&path_handler, toml_string).is_err() {
        // TODO: report error for toml building
        return FAILURE_EXIT_STATUS;
    }

    println!("Success");

    return SUCCESS_EXIT_STATUS;
}

pub fn main(pair: (&str, ArgMatches)) -> i32 {
    let (cmd, matching) = (pair.0, &pair.1);
    let name = ProjectName::from(matching);
    let kind = ProjectKind::from(matching);
    let authors = ProjectAuthors::from(matching);
    let git_repo = !matching.get_flag("no-git");

    if !name.is_valid() {
        report::project::name_is_invalid();
        return FAILURE_EXIT_STATUS;
    }

    if !kind.is_valid() {
        report::project::kind_is_invalid(kind);
        return FAILURE_EXIT_STATUS;
    }

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

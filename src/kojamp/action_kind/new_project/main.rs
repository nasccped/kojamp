use super::{
    fields_validation, file_content, MainToml, ProjectAuthors, ProjectFields, ProjectKind,
    ProjectName, ProjectPath,
};
use crate::{
    globals::{
        GIT_COMMAND, GIT_IGNORE_FILE_FULLNAME, GIT_INITIALIZATION_ARG, JAVA_FILE_EXTENSION,
        KOTLIN_FILE_EXTENSION, MARKDOWN_FILE_EXTENSION, PROGRAM_REPO_URL, PROGRAM_TOML_FILE_NAME,
        README_FILE_NAME, SRC_DIR, TOML_FILE_EXTENSION,
    },
    utils::array::ToText,
};
use clap::ArgMatches;
use colored::Colorize;
use std::{
    fs,
    io::{Error, ErrorKind},
    path::PathBuf,
    process::{Command, Stdio},
};

const INVALID_CUR_DIR: [&str; 7] = [
    "\x1b[91mstd\x1b[97m::\x1b[91menv\x1b[97m::\x1b[91mcurrent_dir()\x1b[0m &&",
    "returned an error!",
    "",
    "The reasons may be:",
    "  \x1b[96ma)\x1b[0m The current path doesn't exists",
    "  \x1b[96mb)\x1b[0m You don't have enough permissions &&",
    "\x1b[3;90m(no sudo or Admin)\x1b[0m",
];

const COULD_NOT_CREATE_PROJECT_DIR: [&str; 2] = [
    "Couldn't create the project directory! (\x1b[92m`kojamp new`\x1b[0m called)",
    "The reason can be due to memory issue.",
];

const COULD_NOT_CREATE_SRC_DIR: [&str; 2] = [
    "Couldn't create src directory!",
    "The reason can be due to memory issue.",
];

const COULD_NOT_CREATE_MAIN_SOURCE_FILE: [&str; 3] = [
    "Couldn't create the main file &&",
    "\x1b[97m(\x1b[91m.java\x1b[97m/\x1b[91m.kt\x1b[97m)\x1b[0m!",
    "The reason can be due to memory issue.",
];

const COULD_NOT_CREATE_TOML: [&str; 6] = [
    "Couldn't create the toml file &&",
    "\x1b[97m(\x1b[91m.java\x1b[97m/\x1b[91m.kt\x1b[97m)\x1b[0m!",
    "The reason can be due to memory issue or error when trying to",
    "unwrap the project fields as TOML String &&",
    "\x1b[3;90m(This last is a lot unexpected, opening an issue &&",
    "is highly encouraged)\x1b[0m",
];

fn create_project_dir(path: &PathBuf) -> Result<(), Error> {
    if fs::create_dir(path).is_err() {
        Err(Error::new(
            ErrorKind::Other,
            COULD_NOT_CREATE_PROJECT_DIR.to_text(),
        ))
    } else {
        Ok(())
    }
}

fn create_src_dir(path: &mut PathBuf) -> Result<(), Error> {
    path.push(SRC_DIR);
    if fs::create_dir(&path).is_err() {
        return Err(Error::new(
            ErrorKind::Other,
            COULD_NOT_CREATE_SRC_DIR.to_text(),
        ));
    }
    path.pop();
    Ok(())
}

fn create_main_source_file(path: &mut PathBuf, fields: &ProjectFields) -> Result<(), Error> {
    let (name, kind) = (fields.get_name().get_inner(), fields.get_kind());
    let (ext, content) = match kind {
        ProjectKind::Java => (JAVA_FILE_EXTENSION, file_content::java(name)),
        _ => (KOTLIN_FILE_EXTENSION, file_content::kotlin(name)),
    };

    path.push(SRC_DIR);
    path.push(name);
    path.set_extension(ext);

    if fs::write(&path, content).is_err() {
        return Err(Error::new(
            ErrorKind::Other,
            COULD_NOT_CREATE_MAIN_SOURCE_FILE.to_text(),
        ));
    }

    path.pop();
    path.pop();
    Ok(())
}

fn create_toml_file(path: &mut PathBuf, fields: &ProjectFields) -> Result<(), Error> {
    path.push(PROGRAM_TOML_FILE_NAME);
    path.set_extension(TOML_FILE_EXTENSION);
    let toml_content: String = MainToml::from(fields).into();

    if fs::write(&path, toml_content).is_err() {
        return Err(Error::new(
            ErrorKind::Other,
            COULD_NOT_CREATE_TOML.to_text(),
        ));
    }

    path.pop();
    Ok(())
}

fn create_readme_file(path: &mut PathBuf, fields: &ProjectFields) {
    path.push(README_FILE_NAME);
    path.set_extension(MARKDOWN_FILE_EXTENSION);
    let readme_content = file_content::readme(fields);

    if fs::write(&path, readme_content).is_err() {
        println!(
            "{}{}{}{} {}",
            "[".bright_white(),
            "WARNING".bright_yellow(),
            "]".bright_white(),
            ":".bright_white(),
            format!(
                "Couldn't create {}.{}",
                README_FILE_NAME, MARKDOWN_FILE_EXTENSION
            )
            .bright_white()
        );
        println!();
        println!("This is probably due to memory issue");
        println!("Even so, the project was created!");
    }
    path.pop();
}

fn initialize_git_and_create_gitignore(path: &mut PathBuf) {
    if Command::new(GIT_COMMAND)
        .args([GIT_INITIALIZATION_ARG, path.to_str().unwrap()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_err()
    {
        println!(
            "{}{}{}{} {}",
            "[".bright_white(),
            "WARNING".bright_yellow(),
            "]".bright_white(),
            ":".bright_white(),
            "Couldn't initialize a git repo".bright_white()
        );
        println!();
        println!("The git program probably doesn't exists at your machine");
        println!("Even so, the project was created!");
        return;
    }

    path.push(GIT_IGNORE_FILE_FULLNAME);
    let gitignore_content = file_content::gitignore();

    if fs::write(&path, gitignore_content).is_err() {
        println!(
            "{}{}{}{} {}",
            "[".bright_white(),
            "WARNING".bright_yellow(),
            "]".bright_white(),
            ":".bright_white(),
            "Couldn't create a .gitignore file".bright_white()
        );
        println!();
        println!("This is probably due to memory issue");
        println!("Even so, the project was created!");
        return;
    }
    path.pop();
}

fn print_success(new_was_called: bool, fields: ProjectFields) {
    println!("{}!", "Success".bright_green());
    println!();
    println!(
        "The `{}` project was successfully created at the {} directory!",
        fields.get_name().get_inner().bright_green(),
        if new_was_called {
            format!(
                "`{}`",
                fields
                    .get_path()
                    .get_inner()
                    .to_str()
                    .unwrap()
                    .bright_yellow()
            )
        } else {
            String::from("current")
        }
    );
    println!();
    println!("You can get help about kojamp at");
    println!(
        "it's official repository: {}",
        PROGRAM_REPO_URL.bright_cyan()
    );
}

pub fn main(pair: (&str, ArgMatches)) -> Result<(), Error> {
    let (cmd, matching) = (pair.0, &pair.1);
    let name = ProjectName::from(matching);
    let kind = ProjectKind::from(matching);
    let (path, new_called) = match (cmd, ProjectPath::try_new()) {
        (_, Err(_)) => return Err(Error::new(ErrorKind::Other, INVALID_CUR_DIR.to_text())),
        ("new", Ok(mut x)) => {
            if x.add_from_matching(matching).is_none() {
                x.add_from_project_name(&name);
            }
            (x, true)
        }
        (_, Ok(x)) => (x, false),
    };

    let tests = [
        fields_validation::name_validation(&name),
        fields_validation::kind_validation(&kind),
        fields_validation::path_validation(&path, new_called),
    ];

    if let Some(x) = tests.into_iter().find(|t| t.is_err()) {
        return x;
    }

    let project_fields: ProjectFields = ProjectFields::new()
        .set_name(name)
        .set_kind(kind)
        .set_path(path)
        .set_authors(ProjectAuthors::from(matching))
        .set_repo(matching.get_flag("no-git"))
        .build();

    let mut path = project_fields.get_path().get_inner();

    if new_called {
        if let Err(x) = create_project_dir(&mut path) {
            return Err(x);
        }
    }

    if let Err(x) = create_src_dir(&mut path) {
        return Err(x);
    }

    if let Err(x) = create_main_source_file(&mut path, &project_fields) {
        return Err(x);
    }

    if let Err(x) = create_toml_file(&mut path, &project_fields) {
        return Err(x);
    }

    create_readme_file(&mut path, &project_fields);

    if project_fields.have_repo() {
        initialize_git_and_create_gitignore(&mut path);
    }

    print_success(new_called, project_fields);

    Ok(())
}

use super::{file_content, MainToml, ProjectFields, ProjectKind};
use crate::utils::array::ToText;
use colored::Colorize;
use std::process::{Command, Stdio};
use std::{
    fs,
    io::{Error, ErrorKind},
};

const COULD_NOT_CREATE_SRC_DIR: [&str; 2] = [
    "Couldn't create src directory!",
    "The reason can be due to memory issue.",
];

const COULD_NOT_CREATE_MAIN_FILE: [&str; 3] = [
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

const COULD_NOT_CREATE_GIT_IGNORE: [&str; 2] = [
    "Couldn't create the .gitignore file!",
    "The reason can be due to memory issue.",
];

pub fn init(project_fields: ProjectFields) -> Result<(), Error> {
    let name = project_fields.get_name().get_inner();
    let kind = project_fields.get_kind();
    let mut path = project_fields.get_path().get_inner();
    let content: String;

    path.push("src");
    if fs::create_dir(&path).is_err() {
        return Err(Error::new(
            ErrorKind::Other,
            COULD_NOT_CREATE_SRC_DIR.to_text(),
        ));
    }

    path.push(name);
    path.set_extension(match kind {
        ProjectKind::Java => {
            content = file_content::java(name);
            "java"
        }
        _ => {
            content = file_content::kotlin(name);
            "kt"
        }
    });

    if fs::write(&path, content).is_err() {
        return Err(Error::new(
            ErrorKind::Other,
            COULD_NOT_CREATE_MAIN_FILE.to_text(),
        ));
    }

    path.pop();
    path.pop();
    path.push("Kojamp");
    path.set_extension("toml");

    let toml_content: String = MainToml::from(&project_fields).into();

    if fs::write(&path, toml_content).is_err() {
        return Err(Error::new(
            ErrorKind::Other,
            COULD_NOT_CREATE_TOML.to_text(),
        ));
    }

    path.pop();

    'git_repo: {
        if !project_fields.have_repo() {
            break 'git_repo;
        }

        if Command::new("git")
            .arg("init")
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
            break 'git_repo;
        }

        path.push(".gitignore");
        let gitignore_content = "# Ignore only the output directory\nout/";

        if fs::write(&path, gitignore_content).is_err() {
            return Err(Error::new(
                ErrorKind::Other,
                COULD_NOT_CREATE_GIT_IGNORE.to_text(),
            ));
        }

        path.pop();
    }

    path.push("README");
    path.set_extension("md");

    if fs::write(
        &path,
        file_content::readme(
            project_fields.get_name().get_inner(),
            project_fields.get_kind(),
            project_fields.get_authors().get_inner(),
        ),
    )
    .is_err()
    {
        println!(
            "{}{}{}{} {}",
            "[".bright_white(),
            "WARNING".bright_yellow(),
            "]".bright_white(),
            ":".bright_white(),
            "Couldn't create README.md".bright_white()
        );
        println!();
        println!("This is probably due to memory issue");
        println!("Even so, the project was created!");
    }

    Ok(())
}

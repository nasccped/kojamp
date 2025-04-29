use super::{file_content, project_fields::ProjectFields, project_kind::ProjectKind};
use crate::utils::array::ToText;
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

    if fs::write(path, content).is_err() {
        return Err(Error::new(
            ErrorKind::Other,
            COULD_NOT_CREATE_MAIN_FILE.to_text(),
        ));
    }

    // TODO: impl the remain of the code (Kojamp.toml, README.md, .gitignore)

    Ok(())
}

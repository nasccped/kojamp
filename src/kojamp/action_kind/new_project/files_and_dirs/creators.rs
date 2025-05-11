use super::content;
use crate::{
    core::{
        contracts::{GetInner, Unpack},
        models::ProjectFields,
    },
    globals::{
        GIT_COMMAND, GIT_IGNORE_FILE_FULLNAME, GIT_INITIALIZATION_ARG, JAVA_FILE_EXTENSION,
        KOTLIN_FILE_EXTENSION, MARKDOWN_FILE_EXTENSION, PROGRAM_TOML_FILE_NAME, README_FILE_NAME,
        SRC_DIR, TOML_FILE_EXTENSION,
    },
};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

pub fn create_project_dir(path: &PathBuf) -> Result<(), ()> {
    fs::create_dir(path).map_err(|_| ())
}

pub fn create_src_dir(path: &mut PathBuf) -> Result<(), PathBuf> {
    path.push(SRC_DIR);
    fs::create_dir(&path).map_err(|_| path.clone())?;
    path.pop();
    Ok(())
}

pub fn create_main_source_file(path: &mut PathBuf, fields: &ProjectFields) -> Result<(), PathBuf> {
    let name = fields.get_name().get_inner();
    let (ext, content) = match From::from(fields.get_kind()) {
        "java" => (JAVA_FILE_EXTENSION, content::java(name)),
        _ => (KOTLIN_FILE_EXTENSION, content::kotlin(name)),
    };

    path.push(SRC_DIR);
    path.push(name);
    path.set_extension(ext);
    fs::write(&path, content).map_err(|_| path.clone())?;
    path.pop();
    path.pop();
    Ok(())
}

pub fn create_toml_file(path: &mut PathBuf, fields: &ProjectFields) -> Result<(), PathBuf> {
    path.push(PROGRAM_TOML_FILE_NAME);
    path.set_extension(TOML_FILE_EXTENSION);
    let (name, kind, authors) = fields.unpack();

    let toml_content = content::toml(name, kind, authors);
    fs::write(&path, toml_content).map_err(|_| path.clone())?;
    path.pop();
    Ok(())
}

pub fn create_readme_file(path: &mut PathBuf, fields: &ProjectFields) -> Option<PathBuf> {
    path.push(README_FILE_NAME);
    path.set_extension(MARKDOWN_FILE_EXTENSION);

    let abspath = std::env::current_dir().unwrap().join(&path);

    if abspath.exists() {
        path.pop();
        return None;
    }

    let (name, kind, authors) = fields.unpack();
    let readme_content = content::readme(name, kind, authors);
    let output = fs::write(&path, readme_content).err().map(|_| path.clone());
    path.pop();
    output
}

pub fn initialize_git(path: &PathBuf) -> Option<()> {
    let mut git_abspath = std::env::current_dir().unwrap();
    git_abspath.push(".git");
    if git_abspath.exists() {
        return None;
    }
    Command::new(GIT_COMMAND)
        .args([GIT_INITIALIZATION_ARG, path.to_str().unwrap()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .err()
        .map(|_| ())
}

pub fn create_git_ignore(path: &mut PathBuf) -> Option<PathBuf> {
    path.push(GIT_IGNORE_FILE_FULLNAME);
    let gitignore_content = content::gitignore();
    let output = fs::write(&path, gitignore_content)
        .err()
        .map(|_| path.clone());
    path.pop();
    output
}

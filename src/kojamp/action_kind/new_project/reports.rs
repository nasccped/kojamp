use super::utils::pathbuf_to_str;
use crate::core::reporting::{messages, KojampReport, ReportType};
use crate::globals::PROGRAM_REPO_URL;
use colored::Colorize;
use std::path::PathBuf;

const INVALID_PROJECT_NAME: &str = "Invalid project name";
const INVALID_PROJECT_KIND: &str = "Invalid project kind";
const INVALID_PROJECT_PATH: &str = "Invalid project path";
const COULD_NOT_GET_THE_CURRENT_DIRECTORY: &str = "Couldn't get the current directory";
const COULD_NOT_READ_PROJECT_FOLDER: &str = "Couldn't read project folder";
const NON_EMPTY_DIR: &str = "Non empty dir";
const PROJECT_CREATED: &str = "`$$$` project created";
pub const COULD_NOT_CREATE_PROJECT_DIR: &str = "Couldn't create the project directory";
pub const COULD_NOT_CREATE_SRC_DIR: &str = "Couldn't create the `src` directory";
pub const COULD_NOT_CREATE_MAIN_SRC_FILE: &str = "Couldn't create the main source file";
pub const COULD_NOT_CREATE_TOML_FILE: &str = "Couldn't create the .toml file";
pub const COULD_NOT_CREATE_README_FILE: &str = "Couldn't create README file";
pub const COULD_NOT_INITIALIZE_GIT_REPO: &str = "Couldn't initialize a git repo";
pub const COULD_NOT_CREATE_GITIGNORE: &str = "Couldn't create .gitignore";

pub fn path_error1() -> KojampReport {
    KojampReport::new(
        ReportType::Error,
        COULD_NOT_GET_THE_CURRENT_DIRECTORY,
        messages::invalid_cur_dir(),
    )
}

pub fn name_error(name: &str) -> KojampReport {
    KojampReport::new(
        ReportType::Error,
        INVALID_PROJECT_NAME,
        messages::invalid_project_name(name),
    )
}

pub fn kind_error(kind: &str) -> KojampReport {
    KojampReport::new(
        ReportType::Error,
        INVALID_PROJECT_KIND,
        messages::invalid_project_kind(kind),
    )
}

pub fn path_error2(path: &PathBuf) -> KojampReport {
    KojampReport::new(
        ReportType::Error,
        INVALID_PROJECT_PATH,
        messages::invalid_project_path(path),
    )
}

pub fn dir_file_creation_error(title: &str, path: &PathBuf) -> KojampReport {
    let as_str = pathbuf_to_str(path);
    KojampReport::new(
        ReportType::Error,
        title,
        messages::could_not_create_dir_file(as_str),
    )
}

pub fn dir_file_creation_warning(title: &str, path: &PathBuf) -> KojampReport {
    let as_str = pathbuf_to_str(path);
    KojampReport::new(
        ReportType::Warning,
        title,
        messages::could_not_create_dir_file(as_str),
    )
}
pub fn could_not_read_dir_error() -> KojampReport {
    KojampReport::new(
        ReportType::Error,
        COULD_NOT_READ_PROJECT_FOLDER,
        messages::could_not_read_dir_content(),
    )
}

pub fn non_empty_dir_error() -> KojampReport {
    KojampReport::new(
        ReportType::Error,
        NON_EMPTY_DIR,
        messages::non_empty_dir_initializing(),
    )
}
pub fn git_init_warning() -> KojampReport {
    KojampReport::new(
        ReportType::Warning,
        COULD_NOT_INITIALIZE_GIT_REPO,
        messages::could_not_initialize_git_repo(),
    )
}

pub fn success_report(project_name: &str, new_called: bool, path: &PathBuf) -> KojampReport {
    KojampReport::new(
        ReportType::Success,
        PROJECT_CREATED.replace("$$$", project_name),
        gen_success_message(new_called, path),
    )
}

pub fn gen_success_message(new_called: bool, path: &PathBuf) -> String {
    let mut may_goto = if new_called {
        vec![format!(
            "cd to your project ({})",
            path.to_str().unwrap_or("???").bright_green()
        )]
    } else {
        vec![]
    };

    may_goto.push(format!("use `{}`", "kojamp run".bright_green()));

    let actions = may_goto
        .into_iter()
        .map(|row| format!("... {} ", "*".bright_green()) + row.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "\
        You can now:\n\
        {}
        \n\
        {}: check the official repo ({}) for more details.",
        actions,
        "note".bright_cyan(),
        PROGRAM_REPO_URL
    )
}

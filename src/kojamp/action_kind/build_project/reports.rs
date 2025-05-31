use colored::Colorize;
use std::path::Path;

use crate::core::reporting::{messages, KojampReport, ReportType};

const COULD_NOT_READ_PROJECT_FOLDER: &str = "Couldn't read project folder";
const COULD_NOT_GET_THE_CURRENT_DIRECTORY: &str = "Couldn't get the current directory";
const PROBABLY_A_NON_KOJAMP_PROJECT: &str = "Probably a non kojamp project";
const COULD_NOT_FIND_SRC_DIR: &str = "Couldn't find src dir";
const COULD_NOT_READ_TOML_FILE: &str = "Couldn't read toml file";
const COULD_NOT_GET_PROJECT_NAME_FROM_TOML: &str = "Couldn't get project name from toml";
const COULD_NOT_GET_PROJECT_KIND_FROM_TOML: &str = "Couldn't get project kind from toml";
const UNREADABLE_SRC_CONTENT: &str = "Unreadable src content";
const THERES_NO_FILES_FOR_THE_GIVEN_PROJECT_KIND: &str =
    "There's no files for the given project kind";
const MAIN_PROJECT_FILE_IS_NOT_PRESENT: &str = "Main project file isn't present in src dir";
const COULD_NOT_COMPILE_THE_SOURCES: &str = "Couldn't compile the source code";
const PROGRAM_SUCCESSFULLY_COMPILED: &str = "The `$$$` program was successfully compiled";

pub fn could_not_get_curdir() -> KojampReport {
    KojampReport::new(
        ReportType::Error,
        COULD_NOT_GET_THE_CURRENT_DIRECTORY,
        messages::invalid_cur_dir(),
    )
}

pub fn kojamp_toml_is_missing() -> KojampReport {
    KojampReport::new(
        ReportType::Error,
        PROBABLY_A_NON_KOJAMP_PROJECT,
        messages::kojamp_toml_not_found(),
    )
}

pub fn could_not_read_dir() -> KojampReport {
    KojampReport::new(
        ReportType::Error,
        COULD_NOT_READ_PROJECT_FOLDER,
        messages::could_not_read_dir_content(),
    )
}

pub fn src_dir_is_missing() -> KojampReport {
    KojampReport::new(
        ReportType::Error,
        COULD_NOT_FIND_SRC_DIR,
        messages::empty_message(),
    )
}

pub fn could_not_read_toml_file() -> KojampReport {
    KojampReport::new(
        ReportType::Error,
        COULD_NOT_READ_TOML_FILE,
        messages::toml_file_could_not_be_read(),
    )
}

pub fn could_not_get_project_name_from_toml() -> KojampReport {
    KojampReport::new(
        ReportType::Error,
        COULD_NOT_GET_PROJECT_NAME_FROM_TOML,
        messages::empty_message(),
    )
}

pub fn could_not_get_project_kind_from_toml() -> KojampReport {
    KojampReport::new(
        ReportType::Error,
        COULD_NOT_GET_PROJECT_KIND_FROM_TOML,
        messages::empty_message(),
    )
}

pub fn unreadable_src_content(path: &Path) -> KojampReport {
    KojampReport::new(
        ReportType::Error,
        UNREADABLE_SRC_CONTENT,
        messages::unreadable_src_content(path),
    )
}

pub fn src_dir_is_empty(kind: &str) -> KojampReport {
    KojampReport::new(
        ReportType::Error,
        THERES_NO_FILES_FOR_THE_GIVEN_PROJECT_KIND,
        messages::theres_no_files_for_the_given_project_kind(kind),
    )
}

pub fn main_project_file_is_not_present(file_name: String) -> KojampReport {
    KojampReport::new(
        ReportType::Error,
        MAIN_PROJECT_FILE_IS_NOT_PRESENT,
        messages::main_project_file_is_not_present(file_name),
    )
}

pub fn could_not_compile_the_sources() -> KojampReport {
    KojampReport::new(
        ReportType::Error,
        COULD_NOT_COMPILE_THE_SOURCES,
        messages::empty_message(),
    )
}

pub fn success_report(name: String, file_count: usize) -> KojampReport {
    KojampReport::new(
        ReportType::Success,
        PROGRAM_SUCCESSFULLY_COMPILED.replace("$$$", &format!("{}", name.bright_green())),
        messages::successfully_compiled(file_count),
    )
}

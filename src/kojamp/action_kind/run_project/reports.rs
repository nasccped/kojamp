use crate::core::reporting::{messages, KojampReport, ReportType};
use std::path::Path;

const COULD_NOT_READ_PROJECT_FOLDER: &str = "Couldn't read project folder";
const COULD_NOT_GET_THE_CURRENT_DIRECTORY: &str = "Couldn't get the current directory";
const PROBABLY_A_NON_KOJAMP_PROJECT: &str = "Probably a non kojamp project";
const COULD_NOT_READ_TOML_FILE: &str = "Couldn't read toml file";
const COULD_NOT_GET_PROJECT_NAME_FROM_TOML: &str = "Couldn't get project name from toml";
const COULD_NOT_GET_PROJECT_KIND_FROM_TOML: &str = "Couldn't get project kind from toml";
const OUTPUT_FILE_DOESNT_EXISTS: &str = "Output file doesn't exists";
const COULD_NOT_GENERATE_OUTPUT_FILE_NAME: &str = "Couldn't generate output file name";

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

pub fn output_file_doesnt_exists(file_path: &Path) -> KojampReport {
    KojampReport::new(
        ReportType::Error,
        OUTPUT_FILE_DOESNT_EXISTS,
        messages::output_file_doesnt_exists(file_path),
    )
}

pub fn could_not_generate_output_file_path(name: &str, kind: &str) -> KojampReport {
    KojampReport::new(
        ReportType::Error,
        COULD_NOT_GENERATE_OUTPUT_FILE_NAME,
        messages::could_not_generate_output_file_path(name, kind),
    )
}

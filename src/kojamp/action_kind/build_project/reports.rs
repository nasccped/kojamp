use crate::core::reporting::{messages, KojampReport, ReportType};

const COULD_NOT_READ_PROJECT_FOLDER: &str = "Couldn't read project folder";
const COULD_NOT_GET_THE_CURRENT_DIRECTORY: &str = "Couldn't get the current directory";
const PROBABLY_A_NON_KOJAMP_PROJECT: &str = "Probably a non kojamp project";

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

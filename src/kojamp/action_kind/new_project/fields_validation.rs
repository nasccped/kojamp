use super::{ProjectKind, ProjectName, ProjectPath};
use crate::essentials::report::{
    messages,
    types::{KojampReport, ReportType},
};

const INVALID_PROJECT_NAME_TITLE: &str = "Invalid project name";
const INVALID_PROJECT_KIND_TITLE: &str = "Invalid project kind";
const INVALID_PROJECT_PATH_TITLE: &str = "Invalid project path";

pub fn name_validation(name: &ProjectName) -> Result<(), KojampReport> {
    if !name.is_valid() {
        return Err(KojampReport::new(
            ReportType::Error,
            INVALID_PROJECT_NAME_TITLE,
            messages::invalid_project_name(name.get_inner()),
        ));
    }

    Ok(())
}

pub fn kind_validation(kind: &ProjectKind) -> Result<(), KojampReport> {
    if !kind.is_valid() {
        let kind_value: &str = From::from(kind);
        return Err(KojampReport::new(
            ReportType::Error,
            INVALID_PROJECT_KIND_TITLE,
            messages::invalid_project_kind(kind_value),
        ));
    }

    Ok(())
}

pub fn path_validation(path: &ProjectPath, new_called: bool) -> Result<(), KojampReport> {
    if !path.is_valid(!new_called) {
        return Err(KojampReport::new(
            ReportType::Error,
            INVALID_PROJECT_PATH_TITLE,
            messages::invalid_project_path(&path.get_absolute_path()),
        ));
    }

    Ok(())
}

use super::error_shorthand as quick_err;
use std::{
    borrow::Cow,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

fn path_creation_dry_run<'a>(path: Cow<'a, &'a str>) -> Result<PathBuf, Error> {
    let path = Path::new(path.as_ref());

    if path.file_name().is_none() {
        return Err(quick_err::new_error(
            ErrorKind::InvalidInput,
            "Invalid path name",
        ));
    }

    if path.exists() {
        return Err(quick_err::new_error(
            ErrorKind::AlreadyExists,
            "Already existing path",
        ));
    }

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            return Err(quick_err::new_error(
                ErrorKind::NotFound,
                "Parent path doesn't exists",
            ));
        }
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = parent.metadata()?;
        if metadata.permissions().mode() & 0o200 == 0 {
            return Err(quick_err::new_error(
                ErrorKind::PermissionDenied,
                "Permission denied",
            ));
        }
    }

    Ok(path.to_owned())
}

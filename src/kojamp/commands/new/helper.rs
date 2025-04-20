use clap::ArgMatches;
use std::{fs, path::PathBuf};

pub fn prompt_called(matches: &ArgMatches) -> bool {
    matches.get_flag("prompt")
}

pub fn only_prompt_called(matches: &ArgMatches) -> bool {
    [
        prompt_called(matches),
        matches.get_one::<String>("name").is_none(),
        matches.get_one::<String>("path").is_none(),
        matches.get_one::<String>("type").is_none(),
        matches
            .get_many("authors")
            .map(|vector| vector.cloned().collect::<Vec<String>>())
            .is_none(),
        !matches.get_flag("no-git"),
    ]
    .iter()
    .all(|cond| *cond)
}

pub fn no_args_or_flags(matches: &ArgMatches) -> bool {
    [
        !prompt_called(matches),
        matches.get_one::<String>("name").is_none(),
        matches.get_one::<String>("path").is_none(),
        matches.get_one::<String>("type").is_none(),
        matches
            .get_many("authors")
            .map(|vector| vector.cloned().collect::<Vec<String>>())
            .is_none(),
        !matches.get_flag("no-git"),
    ]
    .iter()
    .all(|cond| *cond)
}

pub enum PathCreationErrors {
    InvalidPathName,
    AlreadyExists,
    ParentDoesntExists,
    #[allow(dead_code)]
    PermissionDenied,
    UndefinedReason,
}

pub fn create_path(path: PathBuf) -> Result<(), PathCreationErrors> {
    if path.file_name().is_none() {
        return Err(PathCreationErrors::InvalidPathName);
    }

    if path.exists() {
        return Err(PathCreationErrors::AlreadyExists);
    }

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            return Err(PathCreationErrors::ParentDoesntExists);
        }
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = parent.metadata()?;
        if metadata.permissions().mode() & 0o200 == 0 {
            return Err(PathCreationErrors::PermissionDenied);
        }
    }

    match fs::create_dir(path) {
        Ok(_) => Ok(()),
        Err(_) => Err(PathCreationErrors::UndefinedReason),
    }
}

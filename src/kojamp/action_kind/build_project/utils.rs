use std::{fs, path::PathBuf};

pub fn file_is_here<T: AsRef<str>>(path: &PathBuf, file: T) -> Result<bool, ()> {
    Ok(fs::read_dir(path)
        .map_err(|_| ())?
        .filter_map(|entry| entry.ok())
        .any(|entry| {
            entry
                .file_name()
                .to_str()
                .map(|name| name == file.as_ref())
                .unwrap_or(false)
        }))
}

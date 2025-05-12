use std::{fs, path::PathBuf};

pub fn pathbuf_to_str(path: &PathBuf) -> &str {
    path.file_name()
        .map(|f| f.to_str().unwrap_or(""))
        .unwrap_or("")
}

pub fn dir_is_empty(path: &PathBuf) -> Result<bool, ()> {
    Ok(fs::read_dir(path).map_err(|_| ())?.next().is_none())
}

pub fn dir_contains<const N: usize>(path: &PathBuf, items: [&str; N]) -> Result<bool, ()> {
    Ok(fs::read_dir(path)
        .map_err(|_| ())?
        .filter_map(|entry| entry.ok())
        .any(|entry| {
            entry
                .file_name()
                .to_str()
                .map(|name| items.contains(&name))
                .unwrap_or(false)
        }))
}

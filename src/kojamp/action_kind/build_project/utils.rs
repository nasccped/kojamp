use std::{fs, path::PathBuf};

pub fn item_is_here<T: AsRef<str>>(path: &PathBuf, file: T) -> Result<bool, ()> {
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

pub fn get_project_name_from_toml(file_input: String) -> Result<String, ()> {
    let toml_file = file_input.parse::<toml::Value>().map_err(|_| ())?;
    let name = toml_file["project"]["name"].as_str().ok_or(())?;
    Ok(String::from(name))
}

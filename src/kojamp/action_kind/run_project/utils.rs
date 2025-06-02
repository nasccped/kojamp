use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

pub fn item_is_here<T: AsRef<str>>(path: &Path, file: T) -> Result<bool, ()> {
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

pub fn get_project_name_from_toml(file_input: &str) -> Result<String, ()> {
    let binding = file_input.parse::<toml::Value>().map_err(|_| ())?;
    let project_table = binding.get("project").ok_or(())?;
    let name_field = project_table.get("name").ok_or(())?.as_str().ok_or(())?;
    Ok(String::from(name_field))
}

pub fn get_project_kind_from_toml(file_input: &str) -> Result<String, ()> {
    let binding = file_input.parse::<toml::Value>().map_err(|_| ())?;
    let project_table = binding.get("project").ok_or(())?;
    let kind_field = project_table.get("kind").ok_or(())?.as_str().ok_or(())?;
    Ok(String::from(kind_field))
}

pub fn try_generate_output_path(name: &str, kind: &str) -> Option<PathBuf> {
    let mut output_name = PathBuf::from("out").join(name);
    let try_setting = match kind {
        "java" => output_name.set_extension("class"),
        "kotlin" => output_name.set_extension("jar"),
        _ => false,
    };
    if try_setting {
        Some(output_name)
    } else {
        None
    }
}

pub fn output_exists(curdir: &Path, output_path: &Path) -> (bool, PathBuf) {
    let abs_path = curdir.join(output_path);
    (abs_path.exists() && abs_path.is_file(), abs_path)
}

pub fn run_output(kind: &str, path: &Path) {
    let mut command = Command::new("java");
    if kind == "kotlin" {
        command.arg("-jar").arg(path);
    } else {
        command
            .args(["--class-path", "out"])
            .arg(path.file_stem().unwrap());
    }
    let _ = command.status();
}

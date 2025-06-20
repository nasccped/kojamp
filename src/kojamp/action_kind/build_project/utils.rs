use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

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

pub fn get_all_sources<T: AsRef<str>>(kind: T, init_path: &Path) -> Result<Vec<PathBuf>, PathBuf> {
    let ext_target = match kind.as_ref() {
        "java" => "java",
        _ => "kt",
    };
    let mut output = Vec::new();

    let files = init_path.read_dir().map_err(|_| init_path)?;

    for f in files {
        let entry = f.map_err(|_| PathBuf::from("UNDEFINED???"))?;
        let abs = entry.path();
        if abs.is_dir() {
            let nested_path = abs.file_name().ok_or(&abs)?;
            let nested_path = init_path.join(nested_path);
            let nested_files = get_all_sources(kind.as_ref(), &nested_path)?;
            output.extend(nested_files.into_iter());
        }
        let f_name = abs.file_name().ok_or(&abs)?;
        match f_name.to_str() {
            None => return Err(PathBuf::from(f_name)),
            Some(x) if x.ends_with(ext_target) => output.push(init_path.join(x)),
            _ => {}
        }
    }

    Ok(output)
}

pub fn run_build(name: &str, sources: Vec<String>, kind: &str) -> Result<bool, Vec<String>> {
    let pathenv: String = env::var("PATH").unwrap_or_default();
    let (cmd_name, dest) = match kind {
        "java" => (String::from("javac"), String::from("out")),
        _ => (
            {
                let mut cmd = String::from("kotlinc");
                if cfg!(target_os = "windows") {
                    cmd.push_str(".bat");
                }
                cmd
            },
            format!("out/{}.jar", name),
        ),
    };

    let mut args = Vec::new();
    if kind == "kotlin" {
        args.push("-include-runtime");
    }
    args.extend(sources.iter().map(|s| s.as_str()));
    args.push("-d");
    args.push(&dest);

    Command::new(cmd_name)
        .env("PATH", pathenv)
        .args(args)
        .status()
        .map(|x| x.success())
        .map_err(|_| sources)
}

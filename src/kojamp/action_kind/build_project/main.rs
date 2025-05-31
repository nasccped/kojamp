use super::{reports::*, utils::*};
use crate::core::reporting::KojampReport;
use std::{fs, path::PathBuf};

const KOJAMP_TOML: &str = "Kojamp.toml";
const SRC_DIR: &str = "src";

pub fn main() -> Result<Vec<KojampReport>, Vec<KojampReport>> {
    let curdir = std::env::current_dir().map_err(|_| vec![could_not_get_curdir()])?;

    match item_is_here(&curdir, KOJAMP_TOML) {
        Ok(false) => Err(vec![kojamp_toml_is_missing()]),
        Err(_) => Err(vec![could_not_read_dir()]),
        _ => Ok(()),
    }?;

    match item_is_here(&curdir, SRC_DIR) {
        Ok(false) => Err(vec![src_dir_is_missing()]),
        Err(_) => Err(vec![could_not_read_dir()]),
        _ => Ok(()),
    }?;

    let toml_content = fs::read_to_string(curdir.join(KOJAMP_TOML))
        .map_err(|_| vec![could_not_read_toml_file()])?;

    let mut errors: Vec<KojampReport> = Vec::new();

    let name = get_project_name_from_toml(&toml_content)
        .map_err(|_| errors.push(could_not_get_project_name_from_toml()));
    let kind = get_project_kind_from_toml(&toml_content)
        .map_err(|_| errors.push(could_not_get_project_kind_from_toml()));

    if !errors.is_empty() {
        return Err(errors);
    }

    let kind = kind.unwrap();
    let name = name.unwrap();
    let src_files = get_all_sources(&kind, &PathBuf::from("src"))
        .map_err(|p| vec![unreadable_src_content(&p)])?;
    let src_files: Vec<String> = src_files
        .into_iter()
        .map(|f| f.to_string_lossy().to_string())
        .collect();

    if src_files.is_empty() {
        return Err(vec![src_dir_is_empty(&kind)]);
    }

    let file_names: Vec<String> = src_files
        .iter()
        .flat_map(|pth| {
            let as_pbf = PathBuf::from(pth);
            as_pbf
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
        })
        .collect();

    let main_file = {
        let mut as_pbf = PathBuf::from(&name);
        as_pbf.set_extension(match kind.as_ref() {
            "java" => "java",
            _ => "kt",
        });
        as_pbf.to_string_lossy().to_string()
    };

    if !file_names.contains(&main_file) {
        return Err(vec![main_project_file_is_not_present(main_file)]);
    }

    match run_build(&name, src_files, &kind) {
        Ok(true) => {}
        _ => return Err(vec![could_not_compile_the_sources()]),
    }

    Ok(vec![success_report(name, file_names.len())])
}

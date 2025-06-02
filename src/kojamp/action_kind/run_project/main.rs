use super::{reports::*, utils::*};
use crate::core::reporting::KojampReport;
use colored::Colorize;
use std::fs;

const KOJAMP_TOML: &str = "Kojamp.toml";

pub fn main() -> Result<Vec<KojampReport>, Vec<KojampReport>> {
    let curdir = std::env::current_dir().map_err(|_| vec![could_not_get_curdir()])?;

    match item_is_here(curdir.as_path(), KOJAMP_TOML) {
        Ok(false) => return Err(vec![kojamp_toml_is_missing()]),
        Err(_) => return Err(vec![could_not_read_dir()]),
        _ => {}
    }

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

    let (name, kind) = (name.unwrap(), kind.unwrap());
    let output_path = try_generate_output_path(name.as_str(), kind.as_str()).ok_or(vec![
        could_not_generate_output_file_path(name.as_str(), kind.as_str()),
    ])?;

    let output_exists = output_exists(curdir.as_path(), output_path.as_path());
    if !output_exists.0 {
        return Err(vec![output_file_doesnt_exists(output_exists.1.as_path())]);
    }

    println!("Running `{}` project:\n", name.bright_green());
    run_output(kind.as_str(), output_exists.1.as_path());

    Ok(vec![])
}

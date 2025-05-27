use super::{reports::*, utils::*};
use crate::core::reporting::{KojampReport, ReportType};
use clap::ArgMatches;
use std::fs;

fn todo_result() -> Vec<KojampReport> {
    vec![KojampReport::new(
        ReportType::Error,
        "Not yet implemented",
        "The code to build project should be implemented",
    )]
}

const KOJAMP_TOML: &str = "Kojamp.toml";
const SRC_DIR: &str = "src";

pub fn main(matching: ArgMatches) -> Result<Vec<KojampReport>, Vec<KojampReport>> {
    let _matching = &matching;
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

    let (_name, _kind) = (name.unwrap(), kind.unwrap());

    // TODO: implement the remaining logic
    Err(todo_result())
}

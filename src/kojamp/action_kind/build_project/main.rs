use super::{reports::*, utils::*};
use crate::core::reporting::{KojampReport, ReportType};
use clap::ArgMatches;

fn todo_result() -> Vec<KojampReport> {
    vec![KojampReport::new(
        ReportType::Error,
        "Not yet implemented",
        "The code to build project should be implemented",
    )]
}

const KOJAMP_TOML: &str = "Kojamp.toml";

pub fn main(matching: ArgMatches) -> Result<Vec<KojampReport>, Vec<KojampReport>> {
    let _matching = &matching;
    let curdir = std::env::current_dir().map_err(|_| vec![could_not_get_curdir()])?;

    match file_is_here(&curdir, KOJAMP_TOML) {
        Ok(false) => return Err(vec![kojamp_toml_is_missing()]),
        Err(_) => return Err(vec![could_not_read_dir()]),
        _ => {}
    }

    Err(todo_result())
}

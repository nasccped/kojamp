use clap::ArgMatches;

use crate::core::reporting::{KojampReport, ReportType};

fn todo_result() -> Vec<KojampReport> {
    vec![KojampReport::new(
        ReportType::Error,
        "Not yet implemented",
        "The code to build project should be implemented",
    )]
}

pub fn main(matching: ArgMatches) -> Result<Vec<KojampReport>, Vec<KojampReport>> {
    let _matching = &matching;
    // TODO: implement logic here

    Err(todo_result())
}

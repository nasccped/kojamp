use clap::ArgMatches;

use crate::core::reporting::{KojampReport, ReportType};

pub fn main(matching: ArgMatches) -> Result<Vec<KojampReport>, Vec<KojampReport>> {
    let _matching = &matching;
    // TODO: implement logic here
    Err(vec![KojampReport::new(
        ReportType::Error,
        "Not yet implemented",
        "The code to build project should be implemented",
    )])
}

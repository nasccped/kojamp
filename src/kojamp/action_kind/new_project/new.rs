use super::project_fields::ProjectFields;
use std::io::Error;

pub fn new(_project_fields: ProjectFields) -> Result<(), Error> {
    println!("Let's build this project!");
    Ok(())
}

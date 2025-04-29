use clap::ArgMatches;
use std::rc::Rc;

pub struct ProjectName(Rc<str>);

impl ProjectName {
    /// Test ProjectName validity for ProjectName value
    pub fn is_valid(&self) -> bool {
        // if empty str
        if self.0.is_empty() {
            return false;
        }

        // get chars
        let mut self_chars = self.0.as_ref().chars();

        // NOTE: unwrap is safe here 'cause we have already checked if inner value is empty
        let next = self_chars.next().unwrap();

        // the first char should be uppercase
        if next.is_lowercase() {
            return false;
        }

        // consume the created chars + check if all in range of acceptable java file name
        self_chars.into_iter().all(|c| c.is_digit(36))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<&ArgMatches> for ProjectName {
    fn from(value: &ArgMatches) -> Self {
        Self(Rc::from(
            value
                .get_one::<String>("name")
                // NOTE: We can unwrap the name 'cause it's a required
                // argument, so will never fail
                .unwrap()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" "),
        ))
    }
}

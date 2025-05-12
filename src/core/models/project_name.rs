use super::super::contracts::{GetInner, IsValid};
use clap::ArgMatches;
use std::rc::Rc;

pub struct ProjectName(Rc<str>);

impl From<&ArgMatches> for ProjectName {
    fn from(value: &ArgMatches) -> Self {
        let value = Rc::from(value.get_one::<String>("name").unwrap().as_ref());
        Self(value)
    }
}

impl IsValid for ProjectName {
    fn is_valid(&self) -> bool {
        if self.0.is_empty() {
            return false;
        }

        let mut self_chars = self.0.as_ref().chars();
        let begin = self_chars.next().unwrap();

        if !('A'..='Z').contains(&begin) {
            return false;
        }

        self_chars.into_iter().all(|c| c.is_digit(36))
    }
}

impl GetInner for ProjectName {
    type Output<'a> = &'a str;
    fn get_inner<'a>(&'a self) -> Self::Output<'a> {
        self.0.as_ref()
    }
}

use clap::ArgMatches;
use std::rc::Rc;

pub struct ProjectAuthors(Option<Vec<Rc<str>>>);

impl ProjectAuthors {
    pub fn get_inner(&self) -> Option<Vec<Rc<str>>> {
        self.0.clone()
    }
}

impl From<&ArgMatches> for ProjectAuthors {
    fn from(value: &ArgMatches) -> Self {
        let authors = value.get_one::<String>("authors").map(|aut| {
            aut.split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
                .split(",")
                .map(|name| Rc::from(name.trim().as_ref()))
                .collect()
        });
        Self(authors)
    }
}

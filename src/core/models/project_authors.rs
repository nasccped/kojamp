use super::super::contracts::GetInner;
use clap::ArgMatches;

#[derive(Clone)]
pub struct ProjectAuthors(Vec<String>);

impl TryFrom<&ArgMatches> for ProjectAuthors {
    type Error = ();

    fn try_from(value: &ArgMatches) -> Result<Self, Self::Error> {
        let authors = value.get_one::<String>("authors").ok_or(())?;

        Ok(Self(
            authors
                .split(",")
                .map(|person| person.split_whitespace().collect::<Vec<_>>().join(" "))
                .collect(),
        ))
    }
}

impl GetInner for ProjectAuthors {
    type Output<'a> = Vec<&'a str>;

    fn get_inner<'a>(&'a self) -> Self::Output<'a> {
        self.0.iter().map(|a| a.as_ref()).collect()
    }
}

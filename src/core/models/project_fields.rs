use super::{
    super::contracts::{GetInner, Unpack},
    ProjectAuthors, ProjectKind, ProjectName,
};

pub struct ProjectFields {
    name: ProjectName,
    kind: ProjectKind,
    authors: Option<ProjectAuthors>,
}

impl ProjectFields {
    pub fn new(name: ProjectName, kind: ProjectKind, authors: Option<ProjectAuthors>) -> Self {
        Self {
            name,
            kind,
            authors,
        }
    }

    pub fn get_name(&self) -> &ProjectName {
        &self.name
    }

    pub fn get_kind(&self) -> &ProjectKind {
        &self.kind
    }

    pub fn get_authors(&self) -> &Option<ProjectAuthors> {
        &self.authors
    }
}

impl Unpack for ProjectFields {
    type Output<'a> = (&'a str, &'a str, Option<Vec<String>>);

    fn unpack<'a>(&'a self) -> Self::Output<'a> {
        (
            self.get_name().get_inner(),
            From::from(self.get_kind()),
            self.get_authors()
                .as_ref()
                .map(|authors| authors.get_inner().iter().map(|a| a.to_string()).collect()),
        )
    }
}

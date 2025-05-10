use super::{
    project_authors::ProjectAuthors, project_kind::ProjectKind, project_name::ProjectName,
    project_path::ProjectPath,
};

#[derive(Default)]
pub struct ProjectFieldsInnerBuilder {
    project_name: Option<ProjectName>,
    project_kind: Option<ProjectKind>,
    project_path: Option<ProjectPath>,
    project_authors: Option<ProjectAuthors>,
    project_repo: Option<bool>,
}

impl ProjectFieldsInnerBuilder {
    pub fn set_name(mut self, name: ProjectName) -> Self {
        self.project_name = Some(name);
        self
    }

    pub fn set_kind(mut self, kind: ProjectKind) -> Self {
        self.project_kind = Some(kind);
        self
    }

    pub fn set_authors(mut self, authors: ProjectAuthors) -> Self {
        self.project_authors = Some(authors);
        self
    }

    pub fn set_repo(mut self, repo: bool) -> Self {
        self.project_repo = Some(repo);
        self
    }

    pub fn set_path(mut self, path: ProjectPath) -> Self {
        self.project_path = Some(path);
        self
    }

    pub fn build(self) -> ProjectFields {
        let project_name = self.project_name.expect(
            "Couldn't take ProjectName. You probably missed the ProjectFieldsPrivateBuilder.set_name function",
        );
        let project_kind = self.project_kind.expect(
            "Couldn't take ProjectKind. You probably missed the ProjectFieldsPrivateBuilder.set_kind function",
        );
        let project_path = self.project_path.expect(
            "Couldn't take ProjectPath. You probably missed the ProjectFieldsPrivateBuilder.set_path function",
        );
        let project_authors = self.project_authors.expect(
            "Couldn't take ProjectAuthors. You probably missed the ProjectFieldsPrivateBuilder.set_authors function",
        );
        let project_repo = self.project_repo.expect(
            "Couldn't take bool for project_repo. You probably missed the ProjectFieldsPrivateBuilder.set_repo function",
        );

        ProjectFields {
            project_name,
            project_kind,
            project_path,
            project_authors,
            project_repo,
        }
    }
}

pub struct ProjectFields {
    project_name: ProjectName,
    project_kind: ProjectKind,
    project_path: ProjectPath,
    project_authors: ProjectAuthors,
    project_repo: bool,
}

impl ProjectFields {
    pub fn new() -> ProjectFieldsInnerBuilder {
        ProjectFieldsInnerBuilder::default()
    }

    pub fn get_path(&self) -> &ProjectPath {
        &self.project_path
    }

    pub fn get_name(&self) -> &ProjectName {
        &self.project_name
    }

    pub fn get_kind(&self) -> &ProjectKind {
        &self.project_kind
    }

    pub fn get_authors(&self) -> &ProjectAuthors {
        &self.project_authors
    }

    pub fn have_repo(&self) -> bool {
        self.project_repo
    }
}

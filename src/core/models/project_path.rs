use super::{
    super::contracts::{AddFrom, GetInner, IsValid},
    ProjectName,
};
use crate::utils::string::StringTransformation;
use clap::ArgMatches;
use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

pub struct ProjectPath {
    current_path: PathBuf,
    specified_path: Option<Rc<str>>,
    should_exists: bool,
}

impl ProjectPath {
    pub fn try_new(should_exists: bool) -> Result<Self, ()> {
        let current_path = std::env::current_dir().map_err(|_| ())?;
        let specified_path = None;
        Ok(Self {
            current_path,
            specified_path,
            should_exists,
        })
    }

    pub fn get_absolute_path(&self) -> PathBuf {
        let mut path = self.current_path.clone();
        if let Some(s) = &self.specified_path {
            path.push(s.as_ref());
        }
        path
    }
}

impl TryFrom<&ArgMatches> for ProjectPath {
    type Error = ();

    fn try_from(value: &ArgMatches) -> Result<Self, Self::Error> {
        let current_path = std::env::current_dir().map_err(|_| ())?;
        let specified_path = value
            .get_one::<String>("path")
            .map(|p| Rc::from(p.as_ref()))
            .ok_or(())?;
        let specified_path = Some(specified_path);

        Ok(Self {
            current_path,
            specified_path,
            should_exists: false,
        })
    }
}

impl IsValid for ProjectPath {
    fn is_valid(&self) -> bool {
        match self.current_path.file_name() {
            Some(p) if p != Path::new("") => {}
            _ => return false,
        };

        self.get_absolute_path().exists() == self.should_exists
    }
}

impl AddFrom<&ProjectName> for ProjectPath {
    fn add_from(&mut self, from: &ProjectName) {
        self.specified_path = Some(Rc::from(from.get_inner().to_kebab_case()));
    }
}

impl GetInner for ProjectPath {
    type Output<'a> = PathBuf;

    fn get_inner(&self) -> Self::Output<'_> {
        PathBuf::from(
            self.specified_path
                .as_ref()
                .map(|x| x.as_ref())
                .unwrap_or("."),
        )
    }
}

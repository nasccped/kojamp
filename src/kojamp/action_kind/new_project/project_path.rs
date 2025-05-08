use super::ProjectName;
use crate::utils::string::StringTransformation;
use clap::ArgMatches;
use std::{
    env,
    path::{Path, PathBuf},
    rc::Rc,
};

pub struct ProjectPath {
    current_path: PathBuf,
    specified_path: Option<Rc<str>>,
}

impl ProjectPath {
    pub fn try_new() -> Result<Self, ()> {
        let curdir = env::current_dir().map_err(|_| ())?;

        Ok(Self {
            current_path: curdir,
            specified_path: None,
        })
    }

    pub fn is_valid(&self, should_exists: bool) -> bool {
        let path = if let Some(x) = self.current_path.file_name() {
            x
        } else {
            return false;
        };

        if path == Path::new("") {
            return false;
        }

        self.get_absolute_path().exists() == should_exists
    }

    pub fn get_absolute_path(&self) -> PathBuf {
        let mut path = self.current_path.clone();

        if let Some(s) = &self.specified_path {
            path.push(s.as_ref());
        }

        path
    }

    pub fn add_from_matching(&mut self, matching: &ArgMatches) -> Option<()> {
        let specified = matching.get_one::<String>("path")?;
        self.specified_path = Some(Rc::from(specified.as_ref()));
        Some(())
    }

    pub fn add_from_project_name(&mut self, project_name: &ProjectName) {
        let value = project_name.as_str().to_kebab_case();
        self.specified_path = Some(Rc::from(value))
    }

    pub fn get_inner(&self) -> PathBuf {
        let path = self
            .specified_path
            .as_ref()
            .map(|x| x.as_ref())
            .unwrap_or(".");

        PathBuf::from(path)
    }
}

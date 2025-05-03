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
        match env::current_dir() {
            Ok(x) => Ok(Self {
                current_path: x,
                specified_path: None,
            }),
            _ => Err(()),
        }
    }

    pub fn is_valid(&self, should_exists: bool) -> bool {
        let mut path = self.current_path.clone();

        if path.file_name().is_none() {
            return false;
        }

        let parent = path.parent();

        if parent.is_none() {
            return false;
        }

        let parent = parent.unwrap();

        if !parent.exists() {
            return false;
        }

        if parent == Path::new("") {
            return false;
        }

        if let Some(spc_pth) = self.specified_path.as_ref() {
            path.push(spc_pth.as_ref());
        }

        path.exists() == should_exists
    }

    pub fn add_from_matching(&mut self, matching: &ArgMatches) -> Option<()> {
        if let Some(path) = matching.get_one::<String>("path") {
            self.specified_path = Some(Rc::from(path.as_ref()));
            Some(())
        } else {
            None
        }
    }

    pub fn add_from_project_name(&mut self, project_name: &ProjectName) {
        self.specified_path = Some(Rc::from(project_name.as_str().to_kebab_case()))
    }

    pub fn get_inner(&self) -> PathBuf {
        let path = match &self.specified_path {
            Some(x) => x.as_ref(),
            _ => "",
        };
        PathBuf::from(path)
    }
}

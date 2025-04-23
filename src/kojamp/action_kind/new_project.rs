use crate::utils::{report, string::StringTransformation};
use clap::ArgMatches;
use colored::Colorize;
use std::{env, path::PathBuf};

struct ProjectName<'a>(&'a String);

impl<'a> ProjectName<'a> {
    fn new(matching: &'a ArgMatches) -> Self {
        // NOTE: We can unwrap the name 'cause it's a required
        // argument, so will never fail
        Self(matching.get_one::<String>("name").unwrap())
    }
}

struct ProjectPath(PathBuf);

impl ProjectPath {
    fn from_path_buf(path_buf: PathBuf) -> Self {
        Self(path_buf)
    }

    fn add_from_matching<'a>(&mut self, matching: &'a ArgMatches) -> Option<&'a String> {
        match matching.get_one("path") {
            Some(x) => {
                self.0 = self.0.join(x);
                Some(x)
            }
            _ => None,
        }
    }

    fn add_from_project_name<'a>(&mut self, project_name: &'a ProjectName) {
        self.0 = self.0.join(project_name.0.to_kebab_case());
    }
}

pub fn main(pair: (&str, ArgMatches)) -> i32 {
    let cur_path = match env::current_dir() {
        Err(_) => {
            report::path::undefined_cur_dir();
            return 1;
        }
        Ok(p) => p,
    };

    match pair {
        ("new", matching) => from_new(matching, cur_path),
        (_, matching) => from_init(matching, cur_path),
    }
}

fn from_new(matching: ArgMatches, cur_path: PathBuf) -> i32 {
    let name = ProjectName::new(&matching);
    let mut path = ProjectPath::from_path_buf(cur_path);
    if path.add_from_matching(&matching).is_none() {
        path.add_from_project_name(&name);
    }

    println!("Creating the `{}` project on a new", name.0.bright_green());
    println!("directory: `{}`", path.0.to_str().unwrap().bright_yellow());
    0
}

fn from_init(matching: ArgMatches, cur_path: PathBuf) -> i32 {
    let name = ProjectName::new(&matching);
    let path = ProjectPath::from_path_buf(cur_path);
    println!(
        "Creating the `{}` project on the current",
        name.0.bright_green()
    );
    println!("directory: `{}`", path.0.to_str().unwrap().bright_yellow());
    0
}

use crate::utils::report;
use clap::ArgMatches;
use colored::Colorize;
use std::{env, path::PathBuf};

struct ProjectName<'a>(&'a String);

impl<'a> ProjectName<'a> {
    fn new(matching: &'a ArgMatches) -> Self {
        // NOTE: We can unwrap the name 'cause it's a required
        //       argument, so will never fail
        Self(matching.get_one("name").unwrap())
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
    let path = cur_path.join(name.0);
    println!("Creating the `{}` project on a new", name.0.bright_green());
    println!("directory: `{}`", path.to_str().unwrap().bright_yellow());
    0
}

fn from_init(matching: ArgMatches, cur_path: PathBuf) -> i32 {
    let name = ProjectName::new(&matching);
    println!(
        "Creating the `{}` project on the current",
        name.0.bright_green()
    );
    println!(
        "directory: `{}`",
        cur_path.to_str().unwrap().bright_yellow()
    );
    0
}

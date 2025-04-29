use super::action;
use crate::{
    globals::PROGRAM_REPO_URL,
    utils::{array::ToText, error::ErrorPrinting},
};
use clap::{builder::Styles, ArgMatches, Command};
use std::{
    io::{Error, ErrorKind},
    process,
    rc::Rc,
};

type StrAlias = &'static str;
type MatchingAlias = Option<(Rc<str>, ArgMatches)>;

const CREATE_PROJECT_COMMANDS: [&str; 3] = ["new", "init", "ini"];

const UNDEFINED_ERROR: [&str; 8] = [
    "This message serves to alert that the program has",
    "fallen into an \x1b[91munexpected behavior\x1b[0m.",
    "",
    "Please, consider opening an &&",
    "\x1b[91missue\x1b[0m at \x1b[92m&&",
    PROGRAM_REPO_URL,
    "\x1b[0m",
    "Describe your steps to get here.",
];

#[derive(Default)]
pub struct KojampBuilder {
    name: Option<StrAlias>,
    version: Option<StrAlias>,
    about: Option<StrAlias>,
    author: Option<StrAlias>,
    style: Option<Styles>,
}

impl KojampBuilder {
    pub fn set_name(mut self, name: StrAlias) -> Self {
        self.name = Some(name);
        self
    }

    pub fn set_version(mut self, version: StrAlias) -> Self {
        self.version = Some(version);
        self
    }

    pub fn set_about(mut self, about: StrAlias) -> Self {
        self.about = Some(about);
        self
    }

    pub fn set_author(mut self, author: StrAlias) -> Self {
        self.author = Some(author);
        self
    }

    pub fn set_style(mut self, style: Styles) -> Self {
        self.style = Some(style);
        self
    }

    pub fn build(self) -> Command {
        Command::new(self.name.expect("KojampBuilder.name wasn't set!"))
            .bin_name(self.name.expect("KojampBuilder.name wasn't set!"))
            .version(self.version.expect("KojampBuilder.version wasn't set!"))
            .about(self.about.expect("KojampBuilder.about wasn't set!"))
            .author(self.author.expect("KojampBuilder.author wasn't set!"))
            .styles(self.style.expect("KojampBuilder.style wasn't set!"))
    }
}

pub trait KojampApp {
    fn new_app() -> KojampBuilder;
    fn add_subcommand(self, subcommand: Command) -> Self;
    fn get_matching(&self) -> MatchingAlias;
    fn run_kojamp_app(&mut self, matching: MatchingAlias) -> Result<(), Error>;
    fn exit_output(&self, output: Result<(), Error>);
}

impl KojampApp for Command {
    fn new_app() -> KojampBuilder {
        KojampBuilder::default()
    }

    fn add_subcommand(self, subcommand: Command) -> Self {
        self.subcommand(subcommand)
    }

    fn get_matching(&self) -> MatchingAlias {
        self.clone()
            .get_matches()
            .subcommand()
            .map(|(name, sub_matches)| (Rc::from(name), sub_matches.clone()))
    }

    fn run_kojamp_app(&mut self, matching: MatchingAlias) -> Result<(), Error> {
        if matching.is_none() {
            let _ = self.print_help();
            return Ok(());
        }

        let output: Result<(), Error>;

        let matching = matching.unwrap();

        match (matching.0.as_ref(), matching.1) {
            (x, m) if CREATE_PROJECT_COMMANDS.contains(&x) => {
                output = action::new_project((x, m));
            }
            // if matching isn't None and it's different from the matches above, alert:
            _ => {
                output = Err(Error::new(ErrorKind::Other, UNDEFINED_ERROR.to_text()));
            }
        }
        output
    }

    fn exit_output(&self, output: Result<(), Error>) {
        process::exit(match output {
            Ok(_) => 0,
            Err(x) => {
                x.print_error();
                1
            }
        });
    }
}

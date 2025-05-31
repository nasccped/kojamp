use super::action;
use crate::core::{
    consts::exit_status::{FAILURE_EXIT_STATUS, SUCCESS_EXIT_STATUS},
    reporting::{messages, KojampReport, ReportType},
};
use clap::{builder::Styles, ArgMatches, Command};
use std::{process, rc::Rc};

type StrAlias = &'static str;
type MatchingAlias = Option<(Rc<str>, ArgMatches)>;

const CREATE_PROJECT_COMMANDS: [&str; 3] = ["new", "init", "ini"];
const BUILD_PROJECT_COMMANDS: [&str; 2] = ["build", "b"];

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
    fn run_kojamp_app(
        &mut self,
        matching: MatchingAlias,
    ) -> Result<Vec<KojampReport>, Vec<KojampReport>>;
    fn exit_output(&self, output: Result<Vec<KojampReport>, Vec<KojampReport>>);
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

    fn run_kojamp_app(
        &mut self,
        matching: MatchingAlias,
    ) -> Result<Vec<KojampReport>, Vec<KojampReport>> {
        if matching.is_none() {
            let _ = self.print_help();
            return Ok(Vec::new());
        }

        let matching = matching.unwrap();
        let output: Result<Vec<KojampReport>, Vec<KojampReport>> =
            match (matching.0.as_ref(), matching.1) {
                (x, m) if CREATE_PROJECT_COMMANDS.contains(&x) => action::new_project((x, m)),
                (x, _) if BUILD_PROJECT_COMMANDS.contains(&x) => action::build_project(),
                // if matching isn't None and it's different from the matches above, alert:
                _ => Err(Vec::from([KojampReport::new(
                    ReportType::Error,
                    "Undefined error",
                    messages::main_app_undefined_error(),
                )])),
            };
        output
    }

    fn exit_output(&self, output: Result<Vec<KojampReport>, Vec<KojampReport>>) {
        let (reports, exit): (Vec<KojampReport>, i32) = match output {
            Ok(r) => (r, SUCCESS_EXIT_STATUS),
            Err(r) => (r, FAILURE_EXIT_STATUS),
        };

        let mut reports = reports.iter().peekable();

        while let Some(r) = reports.next() {
            println!("{}", r);
            if reports.peek().is_some() {
                println!();
            }
        }

        process::exit(exit);
    }
}

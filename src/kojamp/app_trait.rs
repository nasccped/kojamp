use crate::globals::PROGRAM_REPO_URL;
use clap::{builder::Styles, ArgMatches, Command};
use colored::Colorize;
use std::{process, rc::Rc};

type StrAlias = &'static str;
type MatchingAlias = Option<(Rc<str>, ArgMatches)>;

pub trait KojampCLI {
    fn new_kojamp(app_name: StrAlias) -> Self;
    fn set_version(self, version: StrAlias) -> Self;
    fn set_about(self, about: StrAlias) -> Self;
    fn set_author(self, author: StrAlias) -> Self;
    fn set_style(self, style: Styles) -> Self;
    fn add_subcommand(self, subcommand: Command) -> Self;
    fn get_matching(&self) -> MatchingAlias;
    fn run_app(&mut self, matching: MatchingAlias) -> i32;
    fn exit_output(&self, out_value: i32);
}

impl KojampCLI for Command {
    fn new_kojamp(app_name: StrAlias) -> Self {
        let mut app = Command::new(app_name);
        app.set_bin_name(app_name);
        app
    }

    fn set_version(self, version: StrAlias) -> Self {
        self.version(version)
    }

    fn set_about(self, about: StrAlias) -> Self {
        self.about(about)
    }

    fn set_author(self, author: StrAlias) -> Self {
        self.author(author)
    }

    fn set_style(self, style: Styles) -> Self {
        self.styles(style)
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

    fn run_app(&mut self, matching: MatchingAlias) -> i32 {
        // TODO: impl the run control flow
        let mut output = 0;
        match matching {
            // if matching is None, no arg was provided, so
            None => {
                let _ = self.print_help();
            }
            // if matching isn't None and it's different from the matches above, alert:
            _ => {
                println!(
                    "This message serves to alert that the program has fallen into {}.",
                    "unexpected behavior".bright_red().bold()
                );
                println!();
                println!(
                    "Please, consider opening an {} at {}",
                    "issue".bright_red().bold(),
                    PROGRAM_REPO_URL.bright_green().bold()
                );
                println!("describing your steps to get here.");
                output = 1;
            }
        }
        output
    }

    fn exit_output(&self, out_value: i32) {
        process::exit(out_value);
    }
}

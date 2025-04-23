use clap::{builder::Styles, ArgMatches, Command};
use std::process;

type StrAlias = &'static str;

pub trait KojampCLI {
    fn new_kojamp(app_name: StrAlias) -> Self;
    fn set_version(self, version: StrAlias) -> Self;
    fn set_about(self, about: StrAlias) -> Self;
    fn set_author(self, author: StrAlias) -> Self;
    fn set_style(self, style: Styles) -> Self;
    fn add_subcommand(self, subcommand: Command) -> Self;
    fn get_matching(&self) -> ArgMatches;
    fn run_app(&self, matching: ArgMatches) -> i32;
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

    fn get_matching(&self) -> ArgMatches {
        self.clone().get_matches().clone()
    }

    fn run_app(&self, matching: ArgMatches) -> i32 {
        // TODO: impl the run control flow
        println!("{:?}", matching);
        0
    }

    fn exit_output(&self, out_value: i32) {
        process::exit(out_value);
    }
}

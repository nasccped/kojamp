use clap::{
    builder::{styling, Styles},
    error::ErrorKind,
    ArgMatches, Command, Error as ClapError,
};

use super::kjc;

pub const PROGRAM_NAME: &str = "kojamp";
pub const PROGRAM_VERSION: &str = "0.0.1";
pub const PROGRAM_ABOUT: &str =
    "Some kind of basic 'n academic \x1b[1;31mJava \x1b[37m/ \x1b[33mKotlin\x1b[0m \"project-manager\" tool";
pub const PROGRAM_AUTHOR: &str = "nasccped <pdbt.contact@gmail.com>";

pub struct KojampCLI(Command);

pub fn gen_default_style() -> Styles {
    Styles::styled()
        .header(styling::AnsiColor::Green.on_default().bold())
        .usage(styling::AnsiColor::Green.on_default().bold())
        .literal(styling::AnsiColor::Cyan.on_default().bold())
        .placeholder(styling::AnsiColor::Cyan.on_default())
}

impl KojampCLI {
    pub fn new(name: &'static str) -> Self {
        KojampCLI(Command::new(name))
    }

    fn get_inner_value(&self) -> Command {
        self.0.clone()
    }

    pub fn add_version(self, version: &'static str) -> Self {
        Self(self.0.version(version))
    }

    pub fn add_author(self, author: &'static str) -> Self {
        Self(self.0.author(author))
    }

    pub fn add_styles(&self, default_style: Styles) -> Self {
        Self(self.get_inner_value().styles(default_style))
    }

    pub fn add_about(self, about: &'static str) -> Self {
        Self(self.0.about(about))
    }

    #[allow(dead_code)]
    pub fn add_subcommand(self, sub: Command) -> Self {
        Self(self.0.subcommand(sub))
    }

    fn unexpected_input(&self) {
        println!("The given input(s) \x1b[1;31mcouldn't\x1b[0m be used!");
        println!("Probably due to invalid input format, or undefined subcommand...");
        println!("Consider using \x1b[1;36m`kojamp --help`\x1b[0m");
    }

    fn print_help(&self) {
        let mut inner = self.get_inner_value();
        let _ = inner.print_help();
    }

    fn print_version(&self) {
        let inner = self.get_inner_value();
        let version = inner.get_version();
        println!("kojamp {}", version.unwrap());
    }

    pub fn get_action(&self) -> Result<ArgMatches, ClapError> {
        let inner = self.get_inner_value();
        inner.try_get_matches()
    }

    pub fn run(&self, action: Result<ArgMatches, ClapError>) -> i32 {
        let mut output = 0;

        if let Err(x) = action {
            match x.kind() {
                ErrorKind::DisplayHelp => self.print_help(),
                ErrorKind::DisplayVersion => self.print_version(),
                _ => {
                    self.unexpected_input();
                    output += 1
                }
            }
            return output;
        }

        if std::env::args().count() == 1 {
            self.print_help();
            return output;
        }

        let action = action.unwrap();

        match action.subcommand().unwrap() {
            ("new", new_matching) => {
                output = kjc::action_new(new_matching);
            }
            _ => {
                output += 1;
            }
        }
        output
    }

    pub fn exit_with_output(&self, output: i32) {
        std::process::exit(output);
    }
}

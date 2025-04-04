use clap::{error::ErrorKind, ArgMatches, Command, Error as ClapError};

pub const PROGRAM_NAME: &str = "kojamp";
pub const PROGRAM_VERSION: &str = "0.0.1";
pub const PROGRAM_ABOUT: &str =
    "Some kind of basic 'n academic \x1b[1;31mJava \x1b[37m/ \x1b[33mKotlin\x1b[0m \"project-manager\" tool";
pub const PROGRAM_AUTHOR: &str = "nasccped <pdbt.contact@gmail.com>";

pub struct KojampCLI(Command);

use super::KojampOutput;

pub enum TestCase {
    ItsOk,
    DefinitelyNotOk,
}

impl KojampOutput<TestCase, i32> for TestCase {
    fn new(default: TestCase) -> Self {
        default
    }

    fn update(&mut self, new_value: TestCase) {
        *self = new_value;
    }

    fn log_value(&self) {
        match self {
            TestCase::ItsOk => println!("Everything is ok!"),
            TestCase::DefinitelyNotOk => println!("Everything is wrong  Xd"),
        }
    }

    fn get_value(&self) -> Result<i32, i32> {
        match self {
            TestCase::ItsOk => Ok(0),
            TestCase::DefinitelyNotOk => Err(1),
        }
    }
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

    pub fn add_about(self, about: &'static str) -> Self {
        Self(self.0.about(about))
    }

    #[allow(dead_code)]
    pub fn add_subcommand(self, sub: Command) -> Self {
        Self(self.0.subcommand(sub))
    }

    fn unexpected_input(&self) {
        println!("The given input(s) \x1b[1;31mcouldn't\x1b[0m be used!!");
        println!("Probably due to invalid input format, or undefined subcommand...");
        println!("Consider using \x1b[1;32m`kojamp --help`\x1b[0m");
    }

    fn print_help(&self) {
        let mut inner = self.get_inner_value();
        let _ = inner.print_help();
    }

    fn print_version(&self) {
        let inner = self.get_inner_value();
        inner.render_version();
    }

    pub fn get_action(&self) -> Result<ArgMatches, ClapError> {
        let inner = self.get_inner_value();
        inner.try_get_matches()
    }

    pub fn run(&self, action: Result<ArgMatches, ClapError>) -> impl KojampOutput<TestCase, i32> {
        let output = match action {
            Ok(_) => {
                let inner_output = TestCase::new(TestCase::ItsOk);
                inner_output.log_value();
                inner_output
            }
            Err(x) => {
                let mut inner_output = TestCase::new(TestCase::ItsOk);

                match x.kind() {
                    ErrorKind::DisplayHelp => {
                        self.print_help();
                    }
                    ErrorKind::DisplayVersion => {
                        self.print_version();
                    }
                    _ => {
                        self.unexpected_input();
                        inner_output.update(TestCase::DefinitelyNotOk);
                    }
                }
                inner_output
            }
        };
        output
    }

    pub fn exit_with_output(&self, output: impl KojampOutput<TestCase, i32>) {
        match output.get_value() {
            Ok(_) => std::process::exit(0),
            Err(_) => std::process::exit(1),
        }
    }
}

use clap::Command;

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

    pub fn run(&self) -> impl KojampOutput<TestCase, i32> {
        let _unused_inner = self.get_inner_value();
        let mut output = TestCase::new(TestCase::ItsOk);
        println!("Value \x1b[1;32mbefore\x1b[0m update: (0)");
        output.log_value();
        output.update(TestCase::DefinitelyNotOk);
        println!("Value \x1b[1;31mafter\x1b[0m update: (1)");
        output.log_value();
        output
    }

    pub fn return_final_output(&self, output: impl KojampOutput<TestCase, i32>) {
        match output.get_value() {
            Ok(_) => std::process::exit(0),
            Err(_) => std::process::exit(1),
        }
    }
}

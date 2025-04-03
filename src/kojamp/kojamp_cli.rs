use clap::Command;

pub const PROGRAM_NAME: &str = "kojamp";
pub const PROGRAM_VERSION: &str = "0.0.1";
pub const PROGRAM_ABOUT: &str =
    "Some kind of basic 'n academic \x1b[1;31mJava \x1b[37m/ \x1b[33mKotlin\x1b[0m \"project-manager\" tool";
pub const PROGRAM_AUTHOR: &str = "nasccped <pdbt.contact@gmail.com>";

pub struct KojampCLI(Command);

impl KojampCLI {
    pub fn new(name: &'static str) -> Self {
        KojampCLI(Command::new(name))
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

    pub fn run(&self) -> KojampOutStatus {
        let inner = self.0.clone();
        inner.get_matches();
        KojampOutStatus(1)
    }
}

#[allow(dead_code)]
pub struct KojampOutStatus(u8);

#[allow(dead_code)]
impl KojampOutStatus {
    pub fn get_value(&self) -> u8 {
        self.0
    }
}

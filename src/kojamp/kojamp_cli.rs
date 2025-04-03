use clap::Command;

pub const PROGRAM_NAME: &str = "kojamp";
pub const PROGRAM_VERSION: &str = "0.0.1";
pub const PROGRAM_ABOUT: &str =
    "Some kind of basic 'n academic \x1b[1;31mJava\x1b[37m/\x1b[33mKotlin\x1b[0m \"project-manager\" tool";
pub const PROGRAM_AUTHOR: &str = "nasccped <pdbt.contact@gmail.com>";

pub struct KojampCLI(Command);

impl KojampCLI {
    pub fn build(name: &'static str) -> Self {
        KojampCLI(Command::new(name))
    }

    fn take_inner_value(&self) -> Command {
        self.0.clone()
    }

    pub fn add_version(&mut self, version: &'static str) {
        let inner = self.take_inner_value();
        self.0 = inner.version(version);
    }

    pub fn add_author(&mut self, author: &'static str) {
        let inner = self.take_inner_value();
        self.0 = inner.author(author);
    }

    pub fn add_about(&mut self, about: &'static str) {
        let inner = self.take_inner_value();
        self.0 = inner.about(about);
    }

    #[allow(dead_code)]
    pub fn add_subcommand(&mut self, sub: Command) {
        let inner = self.take_inner_value();
        self.0 = inner.subcommand(sub);
    }

    pub fn run(&self) -> KojampOutStatus {
        let inner = self.take_inner_value();
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

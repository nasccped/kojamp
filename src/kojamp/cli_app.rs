use clap::{
    builder::{styling, Styles},
    Command,
};

use super::commands;

pub const PROGRAM_NAME: &str = "kojamp";
pub const PROGRAM_VERSION: &str = "0.0.1";
pub const PROGRAM_ABOUT: &str = "Some kind of basic 'n academic Java/Kotlin project-manager tool";
pub const PROGRAM_AUTHOR: &str = "nasccped, pdbt.contact@gmail.com";

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
        let mut command = Command::new(name);
        command.set_bin_name(PROGRAM_NAME);
        KojampCLI(command)
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

    pub fn add_subcommand(self, sub: Command, style: Styles) -> Self {
        Self(self.0.subcommand(sub.styles(style)))
    }

    pub fn run(&self) -> i32 {
        let mut output = 0;

        let mut app = self.get_inner_value();

        // if no args given
        if std::env::args().count() == 1 {
            let _ = app.print_help().unwrap();
            return 0;
        }

        let matches = app.get_matches();

        match matches.subcommand() {
            Some(("new", new_matching)) => {
                output = commands::new::action(new_matching);
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

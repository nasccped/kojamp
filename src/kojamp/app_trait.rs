use clap::{builder::Styles, Command};

type StrAlias = &'static str;

pub trait KojampCLI {
    fn new_kojamp(app_name: StrAlias) -> Self;
    fn set_version(self, version: StrAlias) -> Self;
    fn set_about(self, about: StrAlias) -> Self;
    fn set_author(self, author: StrAlias) -> Self;
    fn set_style(self, style: Styles) -> Self;
    fn add_subcommand(self, subcommand: Command) -> Self;
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
}

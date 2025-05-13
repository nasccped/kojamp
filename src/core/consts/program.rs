use clap::builder::{styling, Styles};

pub const PROGRAM_NAME: &str = "kojamp";
pub const PROGRAM_VERSION: &str = "0.0.5";
pub const PROGRAM_ABOUT: &str = "Some kind of basic 'n academic Java/Kotlin project-manager tool";
pub const PROGRAM_AUTHOR: &str = "nasccped, pdbt.contact@gmail.com";
pub const PROGRAM_STYLE: Styles = Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Yellow.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());
pub const PROGRAM_REPO_URL: &str = "https://github.com/nasccped/kojamp";

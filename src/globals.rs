use clap::builder::{styling, Styles};

pub const PROGRAM_NAME: &str = "kojamp";
pub const PROGRAM_VERSION: &str = "0.0.1";
pub const PROGRAM_ABOUT: &str = "Some kind of basic 'n academic Java/Kotlin project-manager tool";
pub const PROGRAM_AUTHOR: &str = "nasccped, pdbt.contact@gmail.com";
pub const STYLE: Styles = Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Cyan.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());

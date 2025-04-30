use clap::builder::{styling, Styles};

pub const PROGRAM_NAME: &str = "kojamp";
pub const PROGRAM_VERSION: &str = "0.0.1";
pub const PROGRAM_ABOUT: &str = "Some kind of basic 'n academic Java/Kotlin project-manager tool";
pub const PROGRAM_AUTHOR: &str = "nasccped, pdbt.contact@gmail.com";
pub const PROGRAM_STYLE: Styles = Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Yellow.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());
pub const PROGRAM_REPO_URL: &str = "https://github.com/nasccped/kojamp";

pub const SUCCESS_EXIT_STATUS: i32 = 0;
pub const FAILURE_EXIT_STATUS: i32 = 1;

pub const SRC_DIR: &str = "src";
pub const JAVA_FILE_EXTENSION: &str = "java";
pub const KOTLIN_FILE_EXTENSION: &str = "kt";
pub const PROGRAM_TOML_FILE_NAME: &str = "Kojamp";
pub const TOML_FILE_EXTENSION: &str = "toml";
pub const README_FILE_NAME: &str = "README";
pub const MARKDOWN_FILE_EXTENSION: &str = "md";

pub const GIT_COMMAND: &str = "git";
pub const GIT_INITIALIZATION_ARG: &str = "init";
pub const GIT_IGNORE_FILE_FULLNAME: &str = ".gitignore";

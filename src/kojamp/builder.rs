use super::{app_trait::KojampCLI, subcommands as subcmds};
use crate::globals::*;
use clap::Command;

pub fn kojamp_app() -> Command {
    let subcommands = [
        // TODO: add subcommands here
        subcmds::new::cmd(),
        subcmds::init::cmd(),
    ];

    let mut app = Command::new_kojamp(PROGRAM_NAME)
        .set_version(PROGRAM_VERSION)
        .set_about(PROGRAM_ABOUT)
        .set_author(PROGRAM_AUTHOR)
        .set_style(PROGRAM_STYLE);

    for sub in subcommands {
        app = app.add_subcommand(sub);
    }

    app
}

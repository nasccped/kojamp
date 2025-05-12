use super::{subcommands as subcmds, KojampApp};
use crate::core::consts::program::{
    PROGRAM_ABOUT, PROGRAM_AUTHOR, PROGRAM_NAME, PROGRAM_STYLE, PROGRAM_VERSION,
};
use clap::Command;

pub fn kojamp_app() -> Command {
    let subcommands = [
        // TODO: add subcommands here
        subcmds::new::cmd(),
        subcmds::init::cmd(),
        subcmds::build::cmd(),
        subcmds::run::cmd(),
    ];

    let mut app: Command = Command::new_app()
        .set_name(PROGRAM_NAME)
        .set_version(PROGRAM_VERSION)
        .set_about(PROGRAM_ABOUT)
        .set_author(PROGRAM_AUTHOR)
        .set_style(PROGRAM_STYLE)
        .build();

    for sub in subcommands {
        app = app.add_subcommand(sub);
    }

    app
}

use super::app::KojampCLI;
use super::commands as cmds;
use crate::globals::*;

pub fn kojamp_app() -> KojampCLI {
    let subcommands = vec![cmds::new::command()];

    let mut app = KojampCLI::new(
        PROGRAM_NAME,
        PROGRAM_VERSION,
        PROGRAM_ABOUT,
        PROGRAM_AUTHOR,
        STYLE,
    );

    for sub in subcommands {
        app = app.add_subcommand(sub);
    }

    app
}

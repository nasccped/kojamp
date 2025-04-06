mod kojamp_cli;
mod kojamp_commands;

use kojamp_cli::{
    gen_default_style, KojampCLI, PROGRAM_ABOUT, PROGRAM_AUTHOR, PROGRAM_NAME, PROGRAM_VERSION,
};
use kojamp_commands as kjc;

pub fn build_kojamp() -> KojampCLI {
    let subcommands = [kjc::command_new()];

    let mut app = KojampCLI::new(PROGRAM_NAME)
        .add_styles(gen_default_style())
        .add_version(PROGRAM_VERSION)
        .add_author(PROGRAM_AUTHOR)
        .add_about(PROGRAM_ABOUT);

    for sub in subcommands {
        app = app.add_subcommand(sub);
    }

    app
}

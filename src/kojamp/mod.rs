mod cli_app;
mod commands;

use cli_app::{
    gen_default_style, KojampCLI, PROGRAM_ABOUT, PROGRAM_AUTHOR, PROGRAM_NAME, PROGRAM_VERSION,
};

pub fn build_kojamp() -> KojampCLI {
    let subcommands = [commands::new::command()];

    let mut app = KojampCLI::new(PROGRAM_NAME)
        .add_styles(gen_default_style())
        .add_version(PROGRAM_VERSION)
        .add_author(PROGRAM_AUTHOR)
        .add_about(PROGRAM_ABOUT);

    for sub in subcommands {
        app = app.add_subcommand(sub, gen_default_style());
    }

    app
}

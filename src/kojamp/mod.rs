mod kojamp_cli;
use kojamp_cli::{KojampCLI, PROGRAM_ABOUT, PROGRAM_AUTHOR, PROGRAM_NAME, PROGRAM_VERSION};

pub fn build_kojamp() -> KojampCLI {
    let mut kojamp = KojampCLI::build(PROGRAM_NAME);
    kojamp.add_version(PROGRAM_VERSION);
    kojamp.add_author(PROGRAM_AUTHOR);
    kojamp.add_about(PROGRAM_ABOUT);
    kojamp
}

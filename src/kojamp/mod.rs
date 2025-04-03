mod kojamp_cli;
mod kojamp_output;

use kojamp_cli::{KojampCLI, PROGRAM_ABOUT, PROGRAM_AUTHOR, PROGRAM_NAME, PROGRAM_VERSION};
pub use kojamp_output::KojampOutput;

pub fn build_kojamp() -> KojampCLI {
    KojampCLI::new(PROGRAM_NAME)
        .add_version(PROGRAM_VERSION)
        .add_author(PROGRAM_AUTHOR)
        .add_about(PROGRAM_ABOUT)
}

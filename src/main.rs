mod core;
mod globals;
mod kojamp;
mod utils;

use kojamp::{kojamp_app, KojampApp};

fn main() {
    let mut app = kojamp_app();
    let matching = app.get_matching();
    let output = app.run_kojamp_app(matching);
    app.exit_output(output);
}

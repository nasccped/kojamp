mod core;
mod kojamp;
mod utils;

use kojamp::{kojamp_app, KojampApp};

fn big_warning() {
    println!("\x1b[93mwarning\x1b[0m: this project is no longer maintained.");
    println!();
    println!("JSmoke (https://github.com/nasccped/jsmoke) is/will be the successor!");
}

fn main() {
    big_warning();
    let mut app = kojamp_app();
    let matching = app.get_matching();
    let output = app.run_kojamp_app(matching);
    app.exit_output(output);
}

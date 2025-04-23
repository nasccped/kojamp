use std::process;
mod globals;
mod kojamp;

fn main() {
    let app = kojamp::kojamp_app();
    app.get_matches();
    process::exit(0);
}

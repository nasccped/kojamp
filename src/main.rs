mod globals;
mod kojamp;
mod utils;

fn main() {
    let app = kojamp::kojamp_app();
    let output = app.run();
    app.exit_with_output(output);
}

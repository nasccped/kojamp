mod kojamp;

pub mod utils;

fn main() {
    let main_cli = kojamp::build_kojamp();
    let _ = main_cli.run();
}

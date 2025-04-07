mod kojamp;

pub mod utils;

fn main() {
    let main_cli = kojamp::build_kojamp();
    let out = main_cli.run();
    main_cli.exit_with_output(out);
}

mod kojamp;

pub mod utils;

fn main() {
    let main_cli = kojamp::build_kojamp();
    let action = main_cli.get_action();
    let out = main_cli.run(action);
    main_cli.exit_with_output(out);
}

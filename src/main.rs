mod commands;
mod kojamp_env;

use commands::KojampMainCommandUtils;
use kojamp_env::KojampEnvStruct;

fn main() {
    let envars = KojampEnvStruct::new();
    let kojamp_man = commands::build_main_command(&envars);
    kojamp_man.run_kojamp();
}

mod commands;
mod kojamp_env;
pub mod utils;

use commands::KojampMainCommandUtils;
use kojamp_env::KojampEnvStruct;

const KOJAMP_ENV_VALUES: &str = include_str!("kojamp.env");

fn main() {
    let kojamp_env_mapping = utils::turn_env_into_hashmap(KOJAMP_ENV_VALUES);
    let envars = KojampEnvStruct::new(kojamp_env_mapping);
    let kojamp_man = commands::build_main_command(&envars);
    kojamp_man.run_kojamp();
}

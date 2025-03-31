mod bye;
mod hello;

use bye::{action as bye_action, command as bye_command};
use hello::{action as hello_action, command as hello_command};

use crate::kojamp_env::KojampEnvStruct;
use clap::Command;

pub trait KojampMainCommandUtils {
    fn add_version(self, env_struct: &KojampEnvStruct) -> Self;
    fn add_author(self, env_struct: &KojampEnvStruct) -> Self;
    fn add_about(self, env_struct: &KojampEnvStruct) -> Self;
    fn run_kojamp(self);
}

impl KojampMainCommandUtils for Command {
    fn add_version(self, env_struct: &KojampEnvStruct) -> Self {
        self.version(env_struct.get_cur_version())
    }

    fn add_about(self, env_struct: &KojampEnvStruct) -> Self {
        self.about(env_struct.get_about())
    }

    fn add_author(self, env_struct: &KojampEnvStruct) -> Self {
        self.author(env_struct.get_author())
    }

    fn run_kojamp(self) {
        let sub_match = self.get_matches();

        match sub_match.subcommand() {
            Some(("hello", mtc)) => hello_action(mtc),
            Some(("bye", mtc)) => bye_action(mtc),
            _ => no_subcommand(),
        }
    }
}

pub fn no_subcommand() {
    println!("No subcommand was given!");
    println!("Try using `kojamp --help`");
}

pub fn build_main_command(env_struct: &KojampEnvStruct) -> Command {
    Command::new(env_struct.get_name())
        .add_version(env_struct)
        .add_about(env_struct)
        .add_author(env_struct)
        .subcommand(hello_command())
        .subcommand(bye_command())
}

use clap::{builder::Styles, ArgMatches, Command};

use super::commands::new as cmd_new;

pub struct KojampCLI(Command);

impl KojampCLI {
    pub fn new(
        name: &'static str,
        version: &'static str,
        about: &'static str,
        author: &'static str,
        style: Styles,
    ) -> Self {
        let mut command = Command::new(name)
            .version(version)
            .about(about)
            .author(author)
            .styles(style);
        command.set_bin_name(name);
        KojampCLI(command)
    }

    fn get_inner_value(&self) -> Command {
        self.0.clone()
    }

    pub fn add_subcommand(self, sub: Command) -> Self {
        let mut inner = self.get_inner_value();
        let sub = sub.styles(inner.get_styles().clone());
        inner = inner.subcommand(sub);
        Self(inner)
    }

    pub fn run(&self) -> i32 {
        let mut output = 0;

        let mut app = self.get_inner_value();

        // if no args given
        if std::env::args().count() == 1 {
            let _ = app.print_help().unwrap();
            return 0;
        }

        let matches = app.get_matches();

        match matches.subcommand() {
            Some(("new", new_matching)) => {
                output = cmd_new::action(new_matching);
            }
            _ => {
                output += 1;
            }
        }
        output
    }

    #[allow(dead_code)]
    pub fn get_subcommand_matching(&self, args: Vec<&'static str>) -> ArgMatches {
        if args.len() < 2 {
            panic!(
                "`get_subcommand_matching` was called but no subcommand was given:\n  {:?}",
                args
            );
        }
        let subcommand = args[1];
        let inner = self.get_inner_value();
        let global_match = inner.get_matches_from(args);
        global_match.subcommand_matches(subcommand).unwrap().clone()
    }

    pub fn exit_with_output(&self, output: i32) {
        std::process::exit(output);
    }
}

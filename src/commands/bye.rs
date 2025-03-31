use clap::{Arg, ArgMatches, Command};

pub fn command() -> Command {
    Command::new("bye").arg(
        Arg::new("NAME")
            .long("name")
            .short('n')
            .required(false)
            .help("Say help to someone"),
    )
}

pub fn action(cmd: &ArgMatches) {
    if let Some(name) = cmd.get_one::<String>("NAME") {
        println!("Bye, {}    xD", name);
    } else {
        println!("Bye to... who?");
    }
}

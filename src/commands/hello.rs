use clap::{Arg, ArgMatches, Command};

pub fn command() -> Command {
    Command::new("hello").arg(
        Arg::new("NAME")
            .long("name")
            .short('n')
            .help("Say hello to someone :^D")
            .required(false),
    )
}

pub fn action(subcmd: &ArgMatches) {
    if let Some(name) = subcmd.get_one::<String>("NAME") {
        println!("Hello, {}    xD", name);
    } else {
        println!("You're saying hello, but... there's no one    :^(");
    }
}

use clap::ArgMatches;

pub fn main(pair: (&str, ArgMatches)) -> i32 {
    match pair {
        ("new", _) => println!("Creating a project on a new directory"),
        _ => println!("Creating a project on the current directory"),
    }
    0
}

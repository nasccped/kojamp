use clap::ArgMatches;

pub fn main(pair: (&str, ArgMatches)) -> i32 {
    match pair {
        ("new", matching) => from_new(matching),
        (_, matching) => from_init(matching),
    }
}

fn from_new(matching: ArgMatches) -> i32 {
    let name: Option<&String> = matching.get_one("name");
    println!("The project name is: {:?}", name);
    0
}

fn from_init(matching: ArgMatches) -> i32 {
    let name: Option<&String> = matching.get_one("name");
    println!("The project name is: {:?}", name);
    0
}

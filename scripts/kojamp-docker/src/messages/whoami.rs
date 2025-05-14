use super::title;
use crate::escape::{ApplyEscape, BRIGHT_GREEN, BRIGHT_RED, BRIGHT_WHITE, BRIGHT_YELLOW};

pub fn whoami_message() {
    title("Whoami?", Some(BRIGHT_RED));
    println!(
        "In this container, you are the `{}`, a special person that",
        "kojampuser".apply(BRIGHT_GREEN)
    );
    println!(
        "I've prepared to create, build and run {} based applications.",
        "kojamp".apply(BRIGHT_YELLOW)
    );
    println!();
    println!("You have access to all directories from here");
    println!(
        "({} - `{}`) forward.",
        format!("{}{}", "$".apply(BRIGHT_WHITE), "HOME".apply(BRIGHT_GREEN)),
        "/home/kojampuser/".apply(BRIGHT_GREEN)
    );
}

use super::title;
use crate::escape::{ApplyEscape, BRIGHT_GREEN, BRIGHT_RED};

pub fn volume_message() {
    title("Volume", Some(BRIGHT_GREEN));
    println!(
        "You can use this as a {}, by the way. So, you can create here, and",
        "volume".apply(BRIGHT_GREEN)
    );
    println!("then, copy to your host machine.");
    println!();
    println!(
        "Running the container with volume declaration {} for this",
        "is required".apply(BRIGHT_RED)
    );
    println!("step!");
    println!();
    println!(
        "Check {} if you",
        "https://github.com/nasccped/kojamp/blob/main/INSTALL.md".apply(BRIGHT_GREEN)
    );
    println!("don't know how.");
}

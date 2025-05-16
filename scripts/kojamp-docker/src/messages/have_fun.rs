use super::title;
use crate::escape::{ApplyEscape, BRIGHT_GREEN, BRIGHT_YELLOW};

pub fn have_fun_message() {
    title("Have Fun!!!", None);
    println!(
        "Let's create some projects with {}. You can type `{}` to get",
        "Kojamp".apply(BRIGHT_YELLOW),
        "kojamp".apply(BRIGHT_GREEN)
    );
    println!("the help panel. Or you can also check the INSTALL guide:");
    println!(
        "   {}",
        "https://github.com/nasccped/kojamp/blob/main/INSTALL.md".apply(BRIGHT_GREEN)
    );
}

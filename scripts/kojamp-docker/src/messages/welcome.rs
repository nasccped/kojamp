use super::title;
use crate::escape::{ApplyEscape, BRIGHT_YELLOW};

pub fn welcome_message() {
    title("Welcome", None);
    println!(
        "You're at the {} docker container and this is your guide page!",
        "Kojamp".apply(BRIGHT_YELLOW)
    );
}

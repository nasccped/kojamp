mod have_fun;
mod volume;
mod welcome;
mod whoami;

use crate::escape::{ApplyEscape, BRIGHT_YELLOW};

pub use have_fun::have_fun_message as have_fun;
pub use volume::volume_message as volume;
pub use welcome::welcome_message as welcome;
pub use whoami::whoami_message as whoami;

fn title(t: &str, escape: Option<&str>) {
    println!("{}", t.apply(escape.unwrap_or(BRIGHT_YELLOW)));
    println!(
        "{}\n",
        "=".repeat(70).apply(escape.unwrap_or(BRIGHT_YELLOW))
    );
}

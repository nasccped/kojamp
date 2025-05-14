mod banner;
mod escape;
mod messages;

use banner::BANNER;

fn main() {
    println!("{}", BANNER);
    messages::welcome();
    println!();
    messages::whoami();
    println!();
    messages::volume();
    println!();
    messages::have_fun();
    println!();
}

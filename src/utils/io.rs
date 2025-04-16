use std::{
    fmt,
    io::{self, Write},
};

pub fn remove_lines(go_up: usize) {
    print!("\x1B[{}A", go_up);
    print!("\x1B[0J");
}

pub fn input<T: fmt::Display>(prompt: T) -> String {
    let mut buffer = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    if let Err(_) = io::stdin().read_line(&mut buffer) {
        "".to_string()
    } else {
        normalize_input(buffer)
    }
}

pub fn normalize_input(input: String) -> String {
    input.replace("\n", "").replace("\r", "")
}

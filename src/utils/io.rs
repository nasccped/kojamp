use std::io;

pub fn read_line() -> String {
    let mut buffer = String::new();
    if let Err(_) = io::stdin().read_line(&mut buffer) {
        "".to_string()
    } else {
        normalize_read_line_input(buffer)
    }
}

fn normalize_read_line_input(input: String) -> String {
    input.replace("\n", "").replace("\r", "")
}

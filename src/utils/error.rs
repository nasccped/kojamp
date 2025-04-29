use std::io::Error;

pub trait ErrorPrinting {
    fn print_error(&self);
}

impl ErrorPrinting for Error {
    fn print_error(&self) {
        println!(
            "{}[{}ERROR{}]: {}{}",
            "\x1b[97m",
            "\x1b[91m",
            "\x1b[97m",
            self.kind(),
            "\x1b[0m"
        );
        println!();

        println!("{}", self.to_string());
    }
}

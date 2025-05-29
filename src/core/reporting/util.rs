use colored::Colorize;

pub trait IntoReasons {
    fn into_reasons(self) -> impl Iterator<Item = String>;
}

impl<const N: usize> IntoReasons for [String; N] {
    fn into_reasons(self) -> impl Iterator<Item = String> {
        self.into_iter()
            .zip('a'..='z')
            .map(|(r, ind)| format!("... {} {}", (ind.to_string() + ")").bright_cyan(), r))
    }
}

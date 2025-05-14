pub trait ApplyEscape {
    fn apply<U: Into<String>>(&self, escape: U) -> String;
}

impl<T: AsRef<str>> ApplyEscape for T {
    fn apply<U: Into<String>>(&self, escape: U) -> String {
        let self_value = self.as_ref();
        let escape = escape.into();
        escape + self_value + "\x1b[0m"
    }
}

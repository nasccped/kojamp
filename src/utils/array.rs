pub trait ToText {
    fn to_text(&self) -> &'static str;
}

impl<const N: usize> ToText for [&str; N] {
    fn to_text(&self) -> &'static str {
        Box::leak(self.join("\n").replace("&&\n", "").into_boxed_str())
    }
}

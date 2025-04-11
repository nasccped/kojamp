pub struct StringChecker;

impl StringChecker {
    pub fn chars_in_range(value: &String, range: impl Into<String>) -> bool {
        let as_string = range.into();
        value.chars().all(|c| as_string.contains(c))
    }
}

pub trait ToTitleCase {
    fn to_title_case(self) -> String;
}

impl ToTitleCase for String {
    fn to_title_case(self) -> String {
        if !self.is_empty() {
            self[..1].to_uppercase() + &self[1..].to_lowercase()
        } else {
            self
        }
    }
}

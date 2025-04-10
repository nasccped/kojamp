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

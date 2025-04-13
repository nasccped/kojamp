use regex::Regex;

pub struct StringChecker;

impl StringChecker {
    pub fn chars_in_range(value: &String, range: impl Into<String>) -> bool {
        let as_string = range.into();
        value.chars().all(|c| as_string.contains(c))
    }
}

pub struct StringTransform;

impl StringTransform {
    pub fn to_title_case(input: String) -> String {
        if input.is_empty() {
            input[..1].to_uppercase() + &input[1..].to_lowercase()
        } else {
            input
        }
    }

    pub fn to_kebab_case(input: String) -> String {
        // WARN: This was copy+pasted frm AI, commenting soon...
        let input = input.as_str();

        let re1 = Regex::new(r"([a-z])([A-Z])").unwrap();
        let step1 = re1.replace_all(input, "$1-$2");

        let re2 = Regex::new(r"[_ ]+").unwrap();
        let step2 = re2.replace_all(&step1, "-");

        let re3 = Regex::new(r"[^a-zA-Z0-9-]").unwrap();
        let step3 = re3.replace_all(&step2, "");

        let binding = &step3.to_lowercase();

        let re4 = Regex::new(r"-+").unwrap();
        let result = re4.replace_all(binding, "-");

        result.trim_matches('-').to_string()
    }
}

use regex::Regex;

pub trait StringTransformation {
    fn to_kebab_case(self) -> String;
    fn to_valid_camel_case(self) -> String;
}

impl<T: AsRef<str>> StringTransformation for T {
    fn to_kebab_case(self) -> String {
        let re1 = Regex::new(r"([A-Z])").unwrap();
        let re2 = Regex::new(r"[_\s]+").unwrap();
        let re3 = Regex::new(r"-+").unwrap();

        let input = self.as_ref();

        let step1 = re1.replace_all(input, "-$1").to_string();
        let step2 = re2.replace_all(&step1, "-").to_string();
        let step3 = re3.replace_all(&step2, "-").to_string();

        step3.trim_matches('-').to_ascii_lowercase()
    }

    fn to_valid_camel_case(self) -> String {
        let original = self.as_ref();

        let re = Regex::new(r"([A-Z][a-z]+|\d+|[a-z]+)").unwrap();

        let parts: Vec<&str> = re.find_iter(original).map(|i| i.as_str()).collect();

        parts
            .into_iter()
            .map(|word| word[..1].to_uppercase() + &word[1..])
            .collect()
    }
}

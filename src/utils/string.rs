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
        let valid_radix: String = self
            .as_ref()
            .chars()
            .map(|c| if c.is_digit(36) { c } else { ' ' })
            .collect();

        let mut camel_case: String = valid_radix
            .split_whitespace()
            .map(|word| word[..1].to_uppercase() + word[1..].to_lowercase().as_str())
            .collect();

        let early_returning_conditions = [
            camel_case.is_empty(),
            camel_case.chars().all(|c| ('0'..='9').contains(&c)),
        ];

        if early_returning_conditions.iter().any(|&c| c) {
            return "SomeCoolProject".into();
        }

        let start_index: usize = camel_case
            .chars()
            .position(|c| !('0'..='9').contains(&c))
            .unwrap_or(0);

        if start_index > 0 {
            let charac = camel_case
                .chars()
                .nth(start_index)
                .unwrap()
                .to_uppercase()
                .to_string();
            camel_case.replace_range(start_index..(start_index + 1), charac.as_ref());
        }

        camel_case[start_index..].into()
    }
}

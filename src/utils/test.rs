#[cfg(test)]
mod arg_test {

    use super::super::arg_test::{ArgTestBuilder, ARG_BUILDER};

    #[test]
    fn on_creation() {
        let arg_b = ArgTestBuilder::new("kojamp");
        assert_eq!(ARG_BUILDER, arg_b);
    }

    #[test]
    fn default_arg_testing() {
        let args = ARG_BUILDER.args_from(["some", "arg"]);
        assert_eq!(args, ["kojamp", "some", "arg"]);
    }

    #[test]
    fn option_testing() {
        let args = ARG_BUILDER.args_from(["some", "--options", "being", "passed"]);
        assert_eq!(args, ["kojamp", "some", "--options", "being", "passed"]);
    }

    #[test]
    #[should_panic]
    fn no_args() {
        // this will panic
        let _ = ARG_BUILDER.args_from([]);
    }
}

#[cfg(test)]
mod strings {

    use super::super::strings::{StringChecker, StringTransform};

    #[test]
    fn in_range_expected() {
        let mut range: String = ('a'..='z').collect();
        range.extend('A'..='Z');

        let samples = ["Rust", "Is", "A", "Cool", "Language"]
            .into_iter()
            .map(|val| val.to_string());

        for word in samples {
            assert!(StringChecker::chars_in_range(&word, &range));
        }
    }

    #[test]
    fn not_in_range_expected() {
        let range: String = ('a'..='z').collect();

        let samples =
            ["ALLCAPS", "with space", "numbers123212", "Symbols!"].map(|val| val.to_string());

        for word in samples {
            assert!(!StringChecker::chars_in_range(&word, &range));
        }
    }

    #[test]
    fn kebab_transform() {
        let inputs_and_kebabs = [
            ("SomeValue", "some-value"),
            ("value with spaces", "value-with-spaces"),
            ("So@@me_Symb^^ols!!", "some-symbols"),
        ];

        for (inp, keb) in inputs_and_kebabs {
            assert_eq!(StringTransform::to_kebab_case(inp), keb);
        }
    }

    #[test]
    fn title_case_transform() {
        let inputs_and_titles = [
            ("some", "Some"),
            ("NAME", "Name"),
            ("iS", "Is"),
            ("cOOL", "Cool"),
        ];

        for (inp, tls) in inputs_and_titles {
            assert_eq!(StringTransform::to_title_case(inp), tls);
        }
    }
}

#[cfg(test)]
mod io {

    use super::super::io;

    #[test]
    fn io() {
        let inputs_and_normals = [
            ("Some Line Breaks\n\n\n\n", "Some Line Breaks"),
            ("r slashs \r\r\r", "r slashs "),
        ];

        for (inp, nor) in inputs_and_normals {
            assert_eq!(io::normalize_read_line_input(inp.to_string()), nor);
        }
    }
}

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
    fn string_checker_in_range() {
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
    fn string_checker_not_in_range_expected() {
        let range: String = ('a'..='z').collect();

        let samples =
            ["ALLCAPS", "with space", "numbers123212", "Symbols!"].map(|val| val.to_string());

        for word in samples {
            assert!(!StringChecker::chars_in_range(&word, &range));
        }
    }

    #[test]
    fn string_transform_to_kebab_case() {
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
    fn string_transform_to_title_case() {
        let inputs_and_titles = [
            ("some", "Some"),
            ("NAME", "Name"),
            ("iS", "Is"),
            ("cOOL", "Cool"),
            ("multi word", "Multi Word"),
            ("MULTI WORD AGAIN", "Multi Word Again"),
        ];

        for (inp, tls) in inputs_and_titles {
            assert_eq!(StringTransform::to_title_case(inp), tls);
        }
    }

    #[test]
    fn string_transform_remove_ansi_escape() {
        let in_and_outs = [
            ("\x1b[1;31mHello, red world!\x1b[0m", "Hello, red world!"),
            (
                "escape \x1b[31mon the \x1b[0mmiddle",
                "escape on the middle",
            ),
            (
                "multi \x1b[3;90m\x1b[32mescapes \x1b[0mxD",
                "multi escapes xD",
            ),
        ];

        for (inp, outs) in in_and_outs {
            assert_eq!(StringTransform::remove_ansi_escape(inp), outs);
        }
    }
}

#[cfg(test)]
mod io {

    use super::super::io;
    use crate::{
        utils::io::{IOReporting, ReportStatus},
        vec_dispbox,
    };

    #[test]
    fn io() {
        let inputs_and_normals = [
            ("Some Line Breaks\n\n\n\n", "Some Line Breaks"),
            ("r slashs \r\r\r", "r slashs "),
        ];

        for (inp, nor) in inputs_and_normals {
            assert_eq!(io::normalize_input(inp.to_string()), nor);
        }
    }

    #[test]
    fn io_reporting_how_many_rows() {
        let rows_and_length = [
            (vec!["row 1", "row 2", "row 3"], 3),
            (vec![], 0),
            (vec!["One", "Two", "Three", "Four", "Five"], 5),
        ];

        for (row, len) in rows_and_length {
            let mut io_report = IOReporting::new::<&str>(ReportStatus::Ok, None, vec_dispbox![]);

            for r in row.into_iter() {
                io_report.append_message_line(r);
            }

            assert_eq!(io_report.get_how_many_rows(), len);
        }
    }
}

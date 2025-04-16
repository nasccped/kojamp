use super::project::{ProjectName, ProjectType};
use crate::{kojamp::builder, utils::arg_test::ARG_BUILDER};
use std::borrow::Cow;

mod project_name {

    use super::*;

    #[test]
    fn valid_naming() {
        let app = builder::kojamp_app();
        let matching_cases = [
            ["new", "Foo"],
            ["new", "Bar"],
            ["new", "DoubleWord"],
            ["new", "Number2Name"],
            ["new", "Baz"],
        ];

        for case in matching_cases {
            let args = ARG_BUILDER.args_from(case);
            let matching = app.get_subcommand_matching(args);
            let project_name = ProjectName::new(&matching);
            assert!(
                project_name.is_valid(),
                "Was expecting a valid name, but invalid was returned with `{}` value",
                Cow::<'_, str>::from(&project_name)
            );
        }
    }

    #[test]
    fn invalid_naming() {
        let app = builder::kojamp_app();
        let matching_cases = [
            ["new", "lowercaseInitial"],
            ["new", "5tartingW1thNumb3r"],
            ["new", "Unallowed-Character"],
            ["new", "CharacterÁccênt"],
            ["new", ""],
        ];

        for case in matching_cases {
            let args = ARG_BUILDER.args_from(case);
            let matching = app.get_subcommand_matching(args);
            let project_name = ProjectName::new(&matching);
            assert!(
                !project_name.is_valid(),
                "Was expecting an invalid name, but valid one was returned with `{}` value",
                Cow::<'_, str>::from(&project_name)
            );
        }
    }
}

mod project_type_validation {

    use super::*;

    #[test]
    fn valid_project_type() {
        let app = builder::kojamp_app();
        let matching_cases = [
            ["new", "--type", "java"],
            ["new", "-T", "Java"],
            ["new", "--type", "J"],
            ["new", "-T", "j"],
            ["new", "--type", "kotlin"],
            ["new", "-T", "KoTlIn"],
            ["new", "--type", "K"],
            ["new", "-T", "k"],
        ];

        for case in matching_cases {
            let args = ARG_BUILDER.args_from(case);
            let matching = app.get_subcommand_matching(args);
            let project_type = ProjectType::new(&matching);
            assert!(
                project_type.is_valid(),
                "Was expecting a valid type, but invalid one was returned (`{}`)",
                Cow::<'_, str>::from(&project_type)
            );
        }
    }

    #[test]
    fn invalid_project_type() {
        let app = builder::kojamp_app();
        let matching_cases = [
            ["new", "-T", "NotJava"],
            ["new", "--type", "N"],
            ["new", "-T", "Ja-va"],
            ["new", "--type", "Gotlin"],
            ["new", "-T", "kótlin"],
        ];

        for case in matching_cases {
            let args = ARG_BUILDER.args_from(case);
            let matching = app.get_subcommand_matching(args);
            let project_type = ProjectType::new(&matching);
            assert!(
                !project_type.is_valid(),
                "Was expecting an invalid type, but a valid one was returned (`{}`)",
                Cow::<'_, str>::from(&project_type)
            );
        }
    }
}

mod project_type_asserting {

    use super::*;

    #[test]
    fn expecting_java() {
        let app = builder::kojamp_app();
        let matching_cases = [
            ["new", "--type", "J"],
            ["new", "-T", "j"],
            ["new", "--type", "java"],
            ["new", "-T", "JaVa"],
        ];

        for case in matching_cases {
            let args = ARG_BUILDER.args_from(case);
            let matching = app.get_subcommand_matching(args);
            let p_type = ProjectType::new(&matching);
            assert_eq!(
                p_type,
                ProjectType::Java,
                "A strange type (`{}`) has been returned when Java type was expected",
                Cow::<'_, str>::from(&p_type)
            );
        }
    }

    #[test]
    fn expecting_kotlin() {
        let app = builder::kojamp_app();
        let matching_cases = [
            ["new", "--type", "K"],
            ["new", "-T", "k"],
            ["new", "--type", "kotlin"],
            ["new", "-T", "KoTlIn"],
        ];

        for case in matching_cases {
            let args = ARG_BUILDER.args_from(case);
            let matching = app.get_subcommand_matching(args);
            let p_type = ProjectType::new(&matching);
            assert_eq!(
                p_type,
                ProjectType::Kotlin,
                "A strange type (`{}`) has been returned when Kotlin type was expected",
                Cow::<'_, str>::from(&p_type)
            );
        }
    }

    #[test]
    fn expecting_undefined() {
        let app = builder::kojamp_app();
        let matching_cases = [
            ["new", "--type", "Undefined"],
            ["new", "-T", "WhatTypeIsThis"],
            ["new", "--type", "RustType"],
            ["new", "-T", "9"],
        ];

        for case in matching_cases {
            let args = ARG_BUILDER.args_from(case);
            let matching = app.get_subcommand_matching(args);
            let p_type = ProjectType::new(&matching);
            assert!(
                match p_type {
                    ProjectType::Undefined(_) => true,
                    _ => false,
                },
                "Undefined type was expected but (`{}`) has been returned",
                Cow::<'_, str>::from(&p_type)
            );
        }
    }

    #[test]
    fn expecting_none() {
        let app = builder::kojamp_app();
        let matching_cases = [
            ["new", "RustProject"],
            ["new", "invalid_name_but_who_cares"],
            ["new", ""],
            ["new", "42"],
        ];

        for case in matching_cases {
            let args = ARG_BUILDER.args_from(case);
            let matching = app.get_subcommand_matching(args);
            let p_type = ProjectType::new(&matching);
            assert_eq!(
                p_type,
                ProjectType::None,
                "None type was expected but (`{}`) has been returned",
                Cow::<'_, str>::from(&p_type)
            );
        }
    }
}

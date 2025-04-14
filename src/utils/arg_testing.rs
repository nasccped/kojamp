pub struct ArgTestBuilder {
    #[allow(dead_code)]
    program_name: &'static str,
}

impl ArgTestBuilder {
    pub const fn new(program_name: &'static str) -> Self {
        Self { program_name }
    }

    #[allow(dead_code)]
    pub fn args_from<const N: usize>(&self, args: [&'static str; N]) -> Vec<&'static str> {
        if args.is_empty() {
            panic!("Given `args` is empty: {:?}. This isn't allowed!", args);
        }
        let mut args_vector = Vec::with_capacity(N + 1);
        args_vector.push(self.program_name);
        args_vector.extend(args);
        args_vector
    }
}

#[allow(dead_code)]
pub const ARG_BUILDER: ArgTestBuilder = ArgTestBuilder::new("kojamp");

#[cfg(test)]
mod arg_test_builder {

    use super::ARG_BUILDER;

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

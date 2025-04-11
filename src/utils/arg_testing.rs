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
        let mut args_vector = Vec::with_capacity(N + 1);
        args_vector.push(self.program_name);
        args_vector.extend(args);
        args_vector
    }
}

#[allow(dead_code)]
pub const ARG_BUILDER: ArgTestBuilder = ArgTestBuilder::new("kojamp");

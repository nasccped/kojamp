use std::{
    borrow::Cow,
    fmt,
    io::{self, Write},
};

pub struct IOAsking<'a> {
    question_indicator: Cow<'a, str>,
    question: Cow<'a, str>,
    hint: Cow<'a, str>,
    input_indicator: Cow<'a, str>,
    answered: bool,
}

impl<'a> IOAsking<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(
        question_indicator: T,
        question: T,
        hint: T,
        input_indicator: T,
    ) -> Self {
        Self {
            question_indicator: question_indicator.into(),
            question: question.into(),
            hint: hint.into(),
            input_indicator: input_indicator.into(),
            answered: false,
        }
    }

    pub fn ask(&self) -> String {
        println!(
            "{}{}{} {} {}",
            if self.answered {
                "\x1b[92m"
            } else {
                "\x1b[91m"
            },
            self.question_indicator,
            "\x1b[0m",
            self.question,
            self.hint
        );

        let input_ind = self.input_indicator.clone();

        normalize_input(input(input_ind))
    }

    pub fn update_question<T: Into<Cow<'a, str>>>(&mut self, question: T) {
        self.question = question.into();
    }

    pub fn update_hint<T: Into<Cow<'a, str>>>(&mut self, hint: T) {
        self.hint = hint.into();
    }

    pub fn set_answered(&mut self, value: bool) {
        self.answered = value;
    }

    pub fn print_content(&self) {
        println!(
            "{}{}{} {} {}",
            if self.answered {
                "\x1b[92m"
            } else {
                "\x1b[91m"
            },
            self.question_indicator,
            "\x1b[0m",
            self.question,
            self.hint
        );
    }
}

pub fn remove_lines(go_up: usize) {
    print!("\x1B[{}A", go_up);
    print!("\x1B[0J");
}

fn input<T: fmt::Display>(prompt: T) -> String {
    let mut buffer = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    if let Err(_) = io::stdin().read_line(&mut buffer) {
        "".to_string()
    } else {
        normalize_input(buffer)
    }
}

pub fn normalize_input(input: String) -> String {
    input.replace("\n", "").replace("\r", "")
}

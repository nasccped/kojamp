use super::strings::StringTransform;
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
    input_escape: Cow<'a, str>,
    answered: bool,
}

impl<'a> IOAsking<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(
        question_indicator: T,
        question: T,
        hint: T,
        input_indicator: T,
        input_escape: T,
    ) -> Self {
        Self {
            question_indicator: question_indicator.into(),
            question: question.into(),
            hint: hint.into(),
            input_indicator: input_indicator.into(),
            input_escape: input_escape.into(),
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

        let input_ind = "\x1b[97m".to_string() + self.input_indicator.clone().as_ref() + "\x1b[0m";
        let response_escape_binding = self.input_escape.clone();
        let response_escape = response_escape_binding.as_ref();

        normalize_input(input(input_ind, response_escape))
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

fn input<T, U>(prompt: T, response_escape: U) -> String
where
    T: fmt::Display,
    U: fmt::Display,
{
    let mut buffer = String::new();
    print!("{}{}", prompt, response_escape);
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

pub enum ReportStatus {
    Ok,
    Warn,
    Err,
    None,
}

pub struct IOReporting {
    title_status: ReportStatus,
    title_content: Option<Cow<'static, str>>,
    message: Vec<Box<dyn fmt::Display>>,
}

impl IOReporting {
    pub fn new<T>(
        title_status: ReportStatus,
        title_content: Option<T>,
        message: Vec<Box<dyn fmt::Display>>,
    ) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self {
            title_status,
            title_content: title_content.map(Into::into),
            message,
        }
    }

    pub fn append_message_line<T: fmt::Display + 'static>(&mut self, message: T) {
        self.message.push(Box::new(message));
    }

    pub fn get_how_many_rows(&self) -> usize {
        self.message.len()
    }

    fn get_title(&self) -> String {
        let white_escape = "\x1b[97m";

        let title_tag = match &self.title_status {
            ReportStatus::Ok => format!("{}[{}SUCCESS{}]:", white_escape, "\x1b[92m", white_escape),
            ReportStatus::Warn => {
                format!("{}[{}WARNING{}]:", white_escape, "\x1b[1;93m", white_escape)
            }
            ReportStatus::Err => format!("{}[{}FAIL{}]:", white_escape, "\x1b[1;91m", white_escape),
            ReportStatus::None => format!("{}[N/A]:", white_escape),
        };

        title_tag + " " + self.title_content.clone().unwrap_or(Cow::from("")).as_ref() + "\x1b[0m"
    }

    fn get_title_bar(&self) -> String {
        let mut repeat_value: usize = if let Some(personalized) = &self.title_content {
            StringTransform::remove_ansi_escape(personalized.as_ref()).len() + 1
        } else {
            0
        };

        repeat_value += match self.title_status {
            ReportStatus::Ok => 10,
            ReportStatus::Err => 7,
            ReportStatus::Warn => 10,
            _ => 6,
        };

        format!("{}{}{}", "\x1b[1;97m", "^".repeat(repeat_value), "\x1b[0m")
    }

    pub fn print_content(&self) {
        println!();
        let title = self.get_title();
        let bar = self.get_title_bar();
        println!("{}\n{}", title, bar);
        for row in &self.message {
            println!("{}", row);
        }
        println!();
    }
}

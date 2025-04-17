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

pub enum ReportStatus {
    Ok,
    Warn,
    Err,
    None,
}

pub struct IOReporting {
    title_status: ReportStatus,
    personalized_title: Option<String>,
    append_to_title: Option<String>,
    message: Vec<Box<dyn fmt::Display>>,
}

impl IOReporting {
    pub fn new<T: Into<String>, U: Into<String>>(
        title_status: ReportStatus,
        personalized_title: Option<T>,
        append_to_title: Option<U>,
        message: Vec<Box<dyn fmt::Display>>,
    ) -> Self {
        Self {
            title_status,
            personalized_title: personalized_title.map(Into::into),
            append_to_title: append_to_title.map(Into::into),
            message,
        }
    }

    fn get_title(&self) -> String {
        let title_escape = match &self.title_status {
            ReportStatus::Ok => "\x1b[1;92m",
            ReportStatus::Warn => "\x1b[1;93m",
            ReportStatus::Err => "\x1b[1;91m",
            ReportStatus::None => "\x1b[1;97m",
        };

        let title_content = match (&self.title_status, &self.personalized_title) {
            (x, None) => Cow::from(match x {
                ReportStatus::Ok => "OK",
                ReportStatus::Warn => "WARN",
                ReportStatus::Err => "FAIL",
                ReportStatus::None => "",
            }),
            (_, Some(x)) => Cow::from(x),
        };

        String::from("\x1b[97m[")
            + title_escape
            + title_content.as_ref()
            + "\x1b[97m]: "
            + &self.append_to_title.clone().unwrap_or("".to_string())
            + "\x1b[0m"
    }

    fn get_title_bar(&self) -> String {
        let mut repeat_value: usize;

        if let Some(personalized) = &self.personalized_title {
            repeat_value = personalized.len() + 3;
        } else {
            repeat_value = match &self.title_status {
                ReportStatus::Ok => 5,
                ReportStatus::None => 3,
                _ => 7,
            }
        }

        if let Some(append) = &self.append_to_title {
            repeat_value += append.len() + 1;
        }

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

use std::borrow::Cow;
use std::io::{Error, ErrorKind};

pub fn new_error<'a>(error_kind: ErrorKind, message: impl Into<Cow<'a, str>>) -> Error {
    let message = message.into();

    Error::new(error_kind, message)
}

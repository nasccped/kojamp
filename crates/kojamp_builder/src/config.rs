use toml::Value;

// key index when accessing toml fields
const ACCESS_PACKAGE: &str = "package";
const ACCESS_NAME: &str = "name";
const ACCESS_VERSION: &str = "version";
const ACCESS_AUTHORS: &str = "authors";
const ACCESS_ABOUT: &str = "description";
const ACCESS_REPO_URL: &str = "repository";

/// Program config data (when building a new Clap app).
///
/// It's usefull to prevent code repetition: just write the app data at
/// `Cargo.toml` and then, pass it to the main program via environment variable
/// instead of copy + paste everything.
#[derive(Debug)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub about: String,
    pub author: String,
    pub repo_url: String,
}

/// Error Kinds that the `TryFrom` trait can return when tryig to unwrap
/// program data fields from `toml::Value`.
///
/// ## Example
///
/// A good `Cargo.toml` file is something like:
///
/// ```toml
/// [package]
/// name = "packname"
/// version = "0.1.0"
/// about = "cool description"
/// authors = ["some body@gmail.com"]
/// repository = "https://link-to.repository"
/// ```
#[derive(Debug)]
pub enum ErrorKind<'a> {
    /// When a toml field is missing.
    ///
    /// It stores a Vector of all missing fields (as String). The field name is
    /// recursive also, so this input:
    ///
    /// ```toml
    /// [package]
    /// name = "example"
    /// # expecting (lang = "rust") to be here...
    /// other_items = {
    ///     # expecting (salary = 1000.50) to be here...
    ///
    ///     # ...
    /// ```
    ///
    /// will return a `FieldsNotFound(["package.lang",
    /// "package.other_items.salary"])`.
    FieldsNotFound(Vec<String>),
    /// When a field doesn't have the expected type. It returns the field
    /// name + field in debug format (ie, `NAME<TYPE(VALUE)>`).
    UnexpectedFieldsFormatting(Vec<String>),
    /// When the 'authors' field doesn't have the expected type (List(String)).
    UnexpectedAuthorsFormatting(String),
    /// When `&str` parsing returns an error.
    UnparseableToml(&'a str),
}

impl ErrorKind<'_> {
    /// Prints the self-enum inner variant.
    pub fn report(&self) {
        eprintln!("toml-config parsing returned:\n{self:?}");
    }
}

impl<'a> TryFrom<&'a str> for Config {
    type Error = ErrorKind<'a>;

    /// Try converting a `&str` to a [`Config`].
    ///
    /// It may return an error accordingly to the [`ErrorKind`] variants
    /// description.
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let toml_item: Value = value
            .parse::<Value>()
            .map_err(|_| ErrorKind::UnparseableToml(value))?;
        let package = toml_item
            .get(ACCESS_PACKAGE)
            .ok_or(ErrorKind::FieldsNotFound(vec![ACCESS_PACKAGE.into()]))?;

        let mut missing_fields = Vec::new();
        let mut not_str_fields = Vec::new();
        let mut struct_fields = Vec::new();

        for field in
            [ACCESS_NAME, ACCESS_VERSION, ACCESS_ABOUT, ACCESS_REPO_URL]
        {
            match package.get(field) {
                Some(v) => match v.as_str() {
                    Some(s) => struct_fields.push(s),
                    _ => not_str_fields.push(format!("{field}<{v:?}>")),
                },
                None => {
                    missing_fields.push(format!("{ACCESS_PACKAGE}.{field}"))
                }
            }
        }

        if !missing_fields.is_empty() {
            return Err(ErrorKind::FieldsNotFound(missing_fields));
        } else if !not_str_fields.is_empty() {
            return Err(ErrorKind::UnexpectedFieldsFormatting(not_str_fields));
        }

        let mut struct_fields = struct_fields.iter();

        let mut next = || struct_fields.next().unwrap().to_string();
        let name = next();
        let version = next();
        let author = try_get_author(package)?;
        let about = next();
        let repo_url = next();

        Ok(Self {
            name,
            version,
            about,
            repo_url,
            author,
        })
    }
}

/// A separated `TryFrom` like function to unwrap the designed author string
/// from a `toml::Value`
///
/// ## Example
///
/// Consider the following input:
///
/// ```toml
/// name = "some_name"
/// version = "0.1.0"
/// authors = ["Author 1", "Author 2"]
/// ```
///
/// Even with two different authors, this function will return an `Ok(String)`
/// _(`"Author 1"`)_, since this is an expected `Cargo.toml` format.
///
/// ## Errors
///
/// This function will return an error based on the input. All errors variants
/// and explanation are disposed at [`ErrorKind`].
///
/// ## Extra
///
/// Before returning the `Ok(String)` result, the author name will be changed
/// to a clap acceptable value, (from `"USER_NAME <USER_EMAIL>"` to
/// `"USER_NAME, USER_EMAIL"`). if it fails, the function will return an
/// [`ErrorKind::UnexpectedAuthorsFormatting`].
fn try_get_author(from: &Value) -> Result<String, ErrorKind<'static>> {
    let authors = from
        .get(ACCESS_AUTHORS)
        .ok_or(ErrorKind::FieldsNotFound(vec![ACCESS_AUTHORS.into()]))?;
    let as_array = author_unwrap_or_err(authors.as_array(), authors)?;
    let first_author = author_unwrap_or_err(
        as_array.iter().next().and_then(|i| i.as_str()),
        authors,
    )?;
    let author = first_author.to_string();
    match (author.replace(" <", ", ").replace(">", ""), author) {
        (new, old) if new.len() == old.len() - 1 => Ok(new),
        _ => author_unwrap_or_err(None, authors),
    }
}

#[cfg(not(doctest))]
/// Util function designed to work within the [`try_get_author`] function only!
///
/// ```rust
/// /* Since the `try_get_author` function operates with
///    a lot of Option<T>, we should avoid this: */
/// let value = turn_into_an_option(input)
///     .ok_or(Error::VariantFromString(value.to_string()))?;
/// // and use this instead
/// let value = author_unwrap_or_err(turn_into_an_option(input))?;
/// ```
fn author_unwrap_or_err<T>(
    from: Option<T>,
    toml_value: &Value,
) -> Result<T, ErrorKind<'static>> {
    from.ok_or(ErrorKind::UnexpectedAuthorsFormatting(format!(
        "{toml_value:?}"
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_INPUT: &str = r#"
        [package]
        name = "some_name"
        version = "0.1.0"
        authors = ["SomeBody <some@mail.com>"]
        description = "A cool description, I guess..."
        repository = "https://url.example"
        "#;

    const MISSING_NAME_INPUT: &str = r#"
        [package]
        nombre = "A spanish key isn't valid :^("
        version = "0.1.0"
        authors = ["SomeBody <some@mail.com>"]
        description = "A cool description, I guess..."
        repository = "https://url.example"
        "#;

    const EMPTY_AUTHORS_INPUT: &str = r#"
        [package]
        name = "other name"
        version = "0.1.0"
        authors = []
        description = "A cool description, I guess..."
        repository = "https://url.example"
        "#;

    const WRONG_AUTHOR_FORMAT_INPUT: &str = r#"
        [package]
        name = "other name"
        version = "0.1.0"
        authors = ["This is <An> Invalid <name@mail.com>"]
        description = "A cool description, I guess..."
        repository = "https://url.example"
        "#;

    const EMPTY_INPUT: &str = "";

    const INVALID_FORMATTING_INPUT: &str = r#"
        [package]
        name = "A name"
        version = 12
        authors = ["Kalleb Jhonsons <pseudo@name.com>"]
        description = "A cool description, I guess..."
        repository = "https://url.example"
        "#;

    const FINAL_INPUT: &str = r#"
        [package]
        name = "package.name"
        version = "0.4.2"
        authors = ["nascc <ped@mail.com>"]
        description = "I live within a test world"
        repository = "https://url.example"
        "#;

    #[test]
    fn valid_input() {
        let _ = Config::try_from(VALID_INPUT).unwrap_or_else(|e| {
            e.report();
            panic!();
        });
    }

    #[test]
    fn missing_name_input() {
        let err_val = Config::try_from(MISSING_NAME_INPUT)
            .expect_err("An Err variant was expected but the result is Ok");
        match err_val {
            ErrorKind::FieldsNotFound(v) => {
                assert_eq!(
                    v,
                    ["package.name"],
                    "Only name field must be missing from input"
                )
            }
            e => {
                panic!(
                    "`FieldsNotFound` variant was expected but item is {e:?}"
                )
            }
        }
    }

    #[test]
    fn empty_authors_input() {
        let err_val = Config::try_from(EMPTY_AUTHORS_INPUT)
            .expect_err("An Err variant was expected but the result is Ok");
        match err_val {
            ErrorKind::UnexpectedAuthorsFormatting(a) => {
                assert_eq!(a, "Array([])")
            }
            e => {
                panic!(
                    "`UnexpectedAuthorsFormatting` variant was expected but item is {e:?}"
                )
            }
        }
    }

    #[test]
    fn wrong_author_format_input() {
        let err_val = Config::try_from(WRONG_AUTHOR_FORMAT_INPUT)
            .expect_err("An Err variant was expected but the result is Ok");
        match err_val {
            ErrorKind::UnexpectedAuthorsFormatting(a) => {
                assert_eq!(
                    a,
                    r#"Array([String("This is <An> Invalid <name@mail.com>")])"#
                )
            }
            e => {
                panic!(
                    "`UnexpectedAuthorsFormatting` variant was expected but item is {e:?}"
                )
            }
        }
    }

    #[test]
    fn empty_input() {
        match Config::try_from(EMPTY_INPUT)
            .expect_err("An Err variant was expected but the result is Ok")
        {
            ErrorKind::FieldsNotFound(v) => assert_eq!(
                v,
                ["package"],
                "Only package should be missing from {EMPTY_INPUT}"
            ),
            e => {
                panic!(
                    "`FieldsNotFound` variant was expected but item is {e:?}"
                )
            }
        }
    }

    #[test]
    fn invalid_formatting_input() {
        let config = Config::try_from(INVALID_FORMATTING_INPUT)
            .expect_err("An Err variant was expected but the result is Ok");
        match config {
            ErrorKind::UnexpectedFieldsFormatting(fields) => {
                assert_eq!(fields, ["version<Integer(12)>"])
            }
            e => panic!(
                "UnexpectedFieldsFormatting variant was expected but return was {e:?}"
            ),
        }
    }

    #[test]
    fn final_input() {
        let config = Config::try_from(FINAL_INPUT).unwrap_or_else(|e| {
            panic!("An Ok variant was expected but the result is {e:?}")
        });

        println!("{}", config.author);
        assert_eq!(config.name, "package.name");
        assert_eq!(config.version, "0.4.2");
        assert_eq!(config.about, "I live within a test world");
        assert_eq!(config.author, "nascc, ped@mail.com");
        assert_eq!(config.repo_url, "https://url.example");
    }
}

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
///
/// This enum represents if:
///
/// - a field is missing (ie, `name` field not found);
///
/// - a field have unexpected formatting (ie, an integer where a string was
///   expected);
///
/// - the `authors` field have unexpected formatting (even the Clap app
///   receives only one author, the `authors` at `Cargo.toml` is always an
///   array of strings).
#[derive(Debug)]
pub enum ErrorKind {
    FieldsNotFound(Vec<String>),
    UnexpectedFieldsFormatting(Vec<String>),
    UnexpectedAuthorsFormatting(String),
    UnexpectedTomlFormatting(String),
}

impl ErrorKind {
    pub fn report(&self) {
        eprintln!("toml-config parsing returned {self:?}");
    }
}

impl TryFrom<Value> for Config {
    type Error = ErrorKind;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        // get package or return not found
        let package = value
            .get(ACCESS_PACKAGE)
            .ok_or(ErrorKind::FieldsNotFound(vec![ACCESS_PACKAGE.into()]))?;
        let fields = [
            ACCESS_NAME,
            ACCESS_VERSION,
            ACCESS_ABOUT,
            // ACCESS_AUTHORS - should not be present since 'authors' field is
            // an array instead of a string
            ACCESS_REPO_URL,
        ];
        // extract values from `toml::Value` obj. else return Err(Missing)
        let toml_values = {
            let (success, errors) = fields
                .iter()
                .map(|f| package.get(f).ok_or(format!("{ACCESS_PACKAGE}.{f}")))
                .fold((Vec::new(), Vec::new()), |(mut suc, mut err), item| {
                    match item {
                        Ok(i) => suc.push(i),
                        Err(i) => err.push(i),
                    }
                    (suc, err)
                });
            if errors.is_empty() {
                Ok(success)
            } else {
                Err(ErrorKind::FieldsNotFound(errors))
            }
        }?;
        // unwrap all items as iter of strings, else return
        // Err(UnexpectedFormat)
        let mut toml_strs = {
            let (success, errors) = toml_values
                .iter()
                .map(|v| v.as_str().ok_or(format!("{v:?}")))
                .fold((Vec::new(), Vec::new()), |(mut suc, mut err), item| {
                    match item {
                        Ok(i) => suc.push(i),
                        Err(i) => err.push(i),
                    }
                    (suc, err)
                });
            if errors.is_empty() {
                Ok(success)
            } else {
                Err(ErrorKind::UnexpectedFieldsFormatting(errors))
            }
        }
        .map(|vector| vector.into_iter())?;

        let mut next_str = || toml_strs.next().unwrap();
        let name = next_str().to_string();
        let version = next_str().to_string();
        let author = try_get_author(package)?;
        let about = next_str().to_string();
        let repo_url = next_str().to_string();

        Ok(Self {
            name,
            version,
            about,
            repo_url,
            author,
        })
    }
}

impl TryFrom<&str> for Config {
    type Error = ErrorKind;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(tml) = value.parse::<Value>() {
            Config::try_from(tml)
        } else {
            Err(ErrorKind::UnexpectedTomlFormatting(value.into()))
        }
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
/// `"USER_NAME, USER_EMAIL"`). if it fails, the function will also return an
/// [`ErrorKind::UnexpectedAuthorsFormatting`].
fn try_get_author(from: &Value) -> Result<String, ErrorKind> {
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
) -> Result<T, ErrorKind> {
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

    const FINAL_INPUT: &str = r#"
        [package]
        name = "package.name"
        version = "0.4.2"
        authors = ["nascc <ped@mail.com>"]
        description = "I live within a test world"
        repository = "https://url.example"
        "#;

    fn gen_toml(toml_str: &str) -> Value {
        toml_str
            .parse::<Value>()
            .expect("Failed to unwrap string as toml:")
    }

    // ------------------------------------------------------------------------
    // test try_from using &str inputs
    // ------------------------------------------------------------------------
    mod str_testing {
        use super::*;
        #[test]
        fn valid_input() {
            if let Err(x) = Config::try_from(VALID_INPUT) {
                x.report();
                panic!();
            }
        }

        #[test]
        fn missing_name_input() {
            let err_val = Config::try_from(MISSING_NAME_INPUT).expect_err(
            "An Error type (ErrorKind) was expected but Ok(_) was returned!",
        );
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
            let err_val = Config::try_from(EMPTY_AUTHORS_INPUT).expect_err(
            "An Error type (ErrorKind) was expected but Ok(_) was returned!",
        );
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
            let err_val = Config::try_from(WRONG_AUTHOR_FORMAT_INPUT).expect_err(
            "An Error type (ErrorKind) was expected but Ok(_) was returned!",
        );
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
                .expect_err("An error was expected from empty input")
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
        fn final_input() {
            let config = match Config::try_from(FINAL_INPUT) {
                Ok(c) => c,
                Err(e) => {
                    panic!("A Ok variant was expected but the result is {e:?}")
                }
            };
            println!("{}", config.author);
            assert_eq!(config.name, "package.name");
            assert_eq!(config.version, "0.4.2");
            assert_eq!(config.about, "I live within a test world");
            assert_eq!(config.author, "nascc, ped@mail.com");
            assert_eq!(config.repo_url, "https://url.example");
        }
    }

    // ------------------------------------------------------------------------
    // test try_from using toml::Value inputs
    // ------------------------------------------------------------------------
    mod toml_obj_testing {
        use super::*;
        #[test]
        fn valid_input() {
            let t = gen_toml(VALID_INPUT);
            if let Err(x) = Config::try_from(t) {
                x.report();
                panic!();
            }
        }

        #[test]
        fn missing_name_input() {
            let t = gen_toml(MISSING_NAME_INPUT);
            let err_val = Config::try_from(t).expect_err(
            "An Error type (ErrorKind) was expected but Ok(_) was returned!",
        );
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
            let t = gen_toml(EMPTY_AUTHORS_INPUT);
            let err_val = Config::try_from(t).expect_err(
            "An Error type (ErrorKind) was expected but Ok(_) was returned!",
        );
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
            let t = gen_toml(WRONG_AUTHOR_FORMAT_INPUT);
            let err_val = Config::try_from(t).expect_err(
            "An Error type (ErrorKind) was expected but Ok(_) was returned!",
        );
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
            let t = gen_toml(EMPTY_INPUT);
            match Config::try_from(t)
                .expect_err("An error was expected from empty input")
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
        fn final_input() {
            let t = gen_toml(FINAL_INPUT);
            let config = match Config::try_from(t) {
                Ok(c) => c,
                Err(e) => {
                    panic!("A Ok variant was expected but the result is {e:?}")
                }
            };
            println!("{}", config.author);
            assert_eq!(config.name, "package.name");
            assert_eq!(config.version, "0.4.2");
            assert_eq!(config.about, "I live within a test world");
            assert_eq!(config.author, "nascc, ped@mail.com");
            assert_eq!(config.repo_url, "https://url.example");
        }
    }
}

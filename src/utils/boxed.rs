/// A shorthand builder to generate a vec
/// of Box<Type> where Type implements fmt::Display
///
/// Better than doing:
/// ```
/// Vec::<Box<dyn fmt::Display>>::from([
///     Box::new(23),
///     Box::new("Rust is cool"),
///     Box..new("xD)
/// ]);
/// ```
#[macro_export]
macro_rules! vec_dispbox {
    [] => {
        Vec::<Box<dyn fmt::Display>>::new()
    };

    [$($element:expr),+ $(,)?] => {
        vec![
            $(Box::new($element) as Box<dyn fmt::Display> ), +
        ]
    };
}

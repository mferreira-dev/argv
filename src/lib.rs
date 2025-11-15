use std::collections::HashSet;

/// Represents a flag accepted by the program.
///
/// # Examples
///
/// ```
/// let foo_flag = Flag::new(
///     vec![FOO_FLAG_SHORT, FOO_FLAG_LONG],
///     FlagAction::NoArg(Box::new(|| println!("Foo"))),
/// );
/// ```
pub struct Flag<'a> {
    /// Exhaustive list of all flag variations.
    text: Vec<&'a str>,

    /// Closure to be called when flag is parsed.
    action: FlagAction<'a>,
}

/// Represents the kind of flags the program accepts.
///
/// # Examples
///
/// ```
/// let foo_action = FlagAction::SingleArg(Box::new(|arg| println!("Foo")));
/// ```
pub enum FlagAction<'a> {
    /// Flag accepts no arguments.
    NoArg(Box<dyn Fn() + 'a>),

    /// Flag accepts a single argument.
    SingleArg(Box<dyn FnMut(Option<String>) + 'a>),
}

impl<'a> Flag<'a> {
    pub fn new(text: Vec<&'a str>, action: FlagAction<'a>) -> Self {
        Self { text, action }
    }
}

/// Handles flags passed in to the program according to their ```FlagAction```.
///
/// # Arguments
/// * `args` - Iterator over all arguments passed in when the program was ran.
/// * `flags` - Vector of ```Flag``` representing all possible flags.
///
/// # Returns
/// An instance of ```Result``` representing whether arguments were successfully parsed.
///
/// # Examples
/// ```
/// let result = argv::handle_flags(env::args(), vec![foo_flag, bar_flag]);
/// match result {
///     Ok(_) => println!("Finished handling args"),
///     Err(msg) => eprintln!("{msg}"),
/// }
/// ```
pub fn handle_flags<'a>(
    mut args: impl Iterator<Item = String>,
    flags: Vec<Flag<'a>>,
    invalid_flag_msg: &'static str,
) -> Result<(), &'static str> {
    let mut seen = HashSet::new();

    // Skip first argument.
    args.next();

    while let Some(arg) = args.next() {
        if seen.contains(&arg.as_str()) {
            continue;
        }

        let found = flags
            .iter()
            .find(|ele| ele.text.contains(&arg.as_str()))
            .ok_or(invalid_flag_msg)?;

        match &found.action {
            FlagAction::NoArg(action) => (action)(),
            FlagAction::SingleArg(action) => (action)(args.next()),
        }

        found.text.iter().for_each(|ele| {
            seen.insert(ele);
        });
    }

    Ok(())
}

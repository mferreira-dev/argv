use std::collections::HashSet;

pub struct Flag<'a> {
    text: Vec<&'a str>,
    action: FlagAction<'a>,
}

pub enum FlagAction<'a> {
    NoArg(Box<dyn Fn() + 'a>),
    SingleArg(Box<dyn Fn(Option<String>) + 'a>),
}

impl<'a> Flag<'a> {
    pub fn new(text: Vec<&'a str>, action: FlagAction<'a>) -> Self {
        Self { text, action }
    }
}

pub fn handle_flags<'a>(
    mut args: impl Iterator<Item = String>,
    flags: Vec<Flag<'a>>,
) -> Result<(), &'static str> {
    let mut seen = HashSet::new();

    // Skip first argument.
    args.next();

    while let Some(arg) = args.next() {
        let found = flags
            .iter()
            .find(|ele| ele.text.contains(&arg.as_str()))
            .ok_or("Invalid flag")?;

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

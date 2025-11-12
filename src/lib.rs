pub enum FlagType {
    NoArg,
    SingleArg,
}

pub struct Flag<'a> {
    text: Vec<&'a str>,
    action: Box<dyn Fn() + 'a>,
    kind: FlagType,
}

impl<'a> Flag<'a> {
    pub fn new(text: Vec<&'a str>, action: impl Fn() + 'a, kind: FlagType) -> Self {
        Self { text, action: Box::new(action), kind }
    }
}

pub fn handle_flags<'a>(mut args: impl Iterator<Item = String>, flags: Vec<Flag<'a>>) {
    args.for_each(|arg| println!("{arg}"));
    flags.iter().for_each(|flag| (flag.action)());
}

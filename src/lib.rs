use std::collections::HashSet;

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
        Self {
            text,
            action: Box::new(action),
            kind,
        }
    }
}

pub fn handle_flags<'a>(
    args: impl Iterator<Item = String>,
    flags: Vec<Flag<'a>>,
) -> Result<(), &'static str> {
    let mut seen = HashSet::new();

    for (i, arg) in args.enumerate() {
        if i == 0 {
            continue;
        }

        let found = flags
            .iter()
            .find(|ele| ele.text.contains(&arg.as_str()))
            .ok_or("Invalid flag")?;

        match &found.kind {
            FlagType::NoArg => println!("NoArg"),
            FlagType::SingleArg => println!("SingleArg"),
        }

        found.text.iter().for_each(|ele| {
            seen.insert(ele);
        });
    }

    Ok(())
}

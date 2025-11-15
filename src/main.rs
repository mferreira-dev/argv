use argv::{Flag, FlagAction};
use std::env;

const FOO_FLAG_SHORT: &str = "-f";
const FOO_FLAG_LONG: &str = "--foo";
const BAR_FLAG_SHORT: &str = "-b";
const BAR_FLAG_LONG: &str = "--bar";

const INVALID_FLAG_MSG: &str = "Invalid flag";

fn main() {
    let foo_flag = Flag::new(
        vec![FOO_FLAG_SHORT, FOO_FLAG_LONG],
        FlagAction::NoArg(Box::new(|| println!("foo"))),
    );

    let mut bar_result = String::new();

    let bar_flag = Flag::new(
        vec![BAR_FLAG_SHORT, BAR_FLAG_LONG],
        FlagAction::SingleArg(Box::new(|arg| {
            bar_result = arg.unwrap_or(String::from("nothing"));
            println!("arg: {bar_result}");
        })),
    );

    let result = argv::handle_flags(env::args(), vec![foo_flag, bar_flag], INVALID_FLAG_MSG);

    match result {
        Ok(_) => println!("Finished handling args"),
        Err(msg) => eprintln!("{msg}"),
    }
}

use argv::{Flag, FlagAction};
use std::env;

const HELP_FLAG_SHORT: &str = "-h";
const HELP_FLAG_LONG: &str = "--help";
const ROM_FLAG_SHORT: &str = "-r";
const ROM_FLAG_LONG: &str = "--rom";

fn main() {
    let help_flag = Flag::new(
        vec![HELP_FLAG_SHORT, HELP_FLAG_LONG],
        FlagAction::NoArg(Box::new(|| println!("Help"))),
    );

    let rom_flag = Flag::new(
        vec![ROM_FLAG_SHORT, ROM_FLAG_LONG],
        FlagAction::SingleArg(Box::new(|arg| {
            let arg = arg.unwrap_or(String::from("Nothing"));
            println!("ROM: {arg}")
        })),
    );

    let result = argv::handle_flags(env::args(), vec![help_flag, rom_flag]);

    match result {
        Ok(_) => println!("Finished handling args"),
        Err(msg) => eprintln!("{msg}"),
    }
}

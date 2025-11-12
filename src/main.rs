use argv::{Flag, FlagType};
use std::env;

const HELP_FLAG_SHORT: &str = "-h";
const HELP_FLAG_LONG: &str = "--help";
const ROM_FLAG_SHORT: &str = "-r";
const ROM_FLAG_LONG: &str = "--rom";

fn main() {
    let help_flag = Flag::new(
        vec![HELP_FLAG_SHORT, HELP_FLAG_LONG],
        || println!("Help"),
        FlagType::NoArg,
    );

    let rom_flag = Flag::new(
        vec![ROM_FLAG_SHORT, ROM_FLAG_LONG],
        || println!("ROM"),
        FlagType::SingleArg,
    );

    argv::handle_flags(env::args(), vec![help_flag, rom_flag]);
}

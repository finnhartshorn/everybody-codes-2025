use crate::template::{ec_cli, Day};
use std::process;

pub fn handle(day: Day) {
    if ec_cli::check().is_err() {
        eprintln!("command \"ec-cli\" not found or not callable. Try installing it from https://github.com/finnhartshorn/ec-cli");
        process::exit(1);
    }

    if let Err(e) = ec_cli::download(day) {
        eprintln!("failed to call ec-cli: {e}");
        process::exit(1);
    };
}

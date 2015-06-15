#![cfg_attr(test, allow(dead_code))]

extern crate arguments;

use arguments::Arguments;
use std::{env, process};
use std::fmt::Display;

const USAGE: &'static str = "
Usage: layer [options]

Options:
    --database PATH          SQLite3 database (required).
    --table    NAME          Table to look for area measurements (required).

    --help                   Display this message.
";

macro_rules! print_error(
    ($fmt:expr, $($args:tt)*) => ({
        use std::io::Write;
        ::std::io::stderr().write_fmt(format_args!($fmt, $($args)*)).unwrap()
    });
);

macro_rules! ok(
    ($result:expr) => (match $result {
        Ok(result) => result,
        Err(error) => fail(error),
    });
);

fn main() {
    let Arguments { options, .. } = ok!(arguments::parse(env::args()));
    match options.get_ref::<String>("database") {
        Some(_) => {},
        _ => usage(USAGE),
    }
}

fn fail<T: Display>(error: T) -> ! {
    print_error!("Error: {}.\n", error);
    process::exit(1);
}

fn usage(message: &str) -> ! {
    print_error!("{}\n", message.trim());
    process::exit(0);
}

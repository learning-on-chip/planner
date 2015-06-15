#![cfg_attr(test, allow(dead_code))]

extern crate arguments;
extern crate sqlite;

use arguments::Arguments;
use sqlite::Database;
use std::fmt::Display;
use std::path::Path;
use std::{env, process};

const CORE_LIKE: &'static str = "core%_area";
const L3_LIKE: &'static str = "l3%_area";

const USAGE: &'static str = "
Usage: layer [options]

Options:
    --database PATH          SQLite3 database (required).
    --table    NAME          Table to look for area measurements (required).

    --help                   Display this message.
";

macro_rules! die(
    ($($arg:tt)*) => (fail(format!($($arg)*)));
);

macro_rules! ok(
    ($result:expr) => (match $result {
        Ok(result) => result,
        Err(error) => fail(error),
    });
);

macro_rules! print_error(
    ($($arg:tt)*) => ({
        use std::io::Write;
        ::std::io::stderr().write_fmt(format_args!($($arg)*)).unwrap_or(())
    });
);

fn main() {
    let Arguments { options, .. } = ok!(arguments::parse(env::args()));

    let database = match options.get_ref::<String>("database") {
        Some(database) => ok!(sqlite::open(&Path::new(database))),
        _ => usage(USAGE),
    };
    let (core, l3) = match options.get_ref::<String>("table") {
        Some(table) => (find(&database, table, CORE_LIKE), find(&database, table, L3_LIKE)),
        _ => usage(USAGE),
    };
    println!("Core: {}", core);
    println!("L3: {}", l3);
}

fn find(database: &Database, table: &str, like: &str) -> f64 {
    use sqlite::State;
    let mut statement = ok!(database.prepare(&format!(
        "SELECT name, value FROM `{}` WHERE name LIKE '{}' LIMIT 1;", table, like,
    )));
    match ok!(statement.step()) {
        State::Row => ok!(statement.column::<f64>(0 + 1)),
        _ => die!("failed to find a required value in the table"),
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

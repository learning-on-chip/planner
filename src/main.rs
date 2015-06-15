#![cfg_attr(test, allow(dead_code))]

extern crate arguments;
extern crate sqlite;

mod layout;

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
    --cores    NUMBER        Number of cores (required).

    --help                   Display this message.
";

macro_rules! die(
    ($($arg:tt)*) => (raise!(format!($($arg)*)));
);

macro_rules! ok(
    ($result:expr) => (match $result {
        Ok(result) => result,
        Err(error) => raise!(error),
    });
);

macro_rules! raise(
    ($error:expr) => (return Err(Box::new($error)));
);

macro_rules! usage(
    () => (usage());
);

pub type Result<T> = std::result::Result<T, Box<Display>>;

fn main() {
    start().unwrap_or_else(|error| fail(&*error));
}

fn start() -> Result<()> {
    let arguments = ok!(arguments::parse(env::args()));

    let cores = match arguments.get::<usize>("cores") {
        Some(cores) => cores,
        _ => usage!(),
    };
    let database = match arguments.get::<String>("database") {
        Some(ref database) => ok!(sqlite::open(&Path::new(database))),
        _ => usage!(),
    };
    let (core_area, l3_area) = match arguments.get::<String>("table") {
        Some(ref table) => {
            (ok!(find(&database, table, CORE_LIKE)), ok!(find(&database, table, L3_LIKE)))
        },
        _ => usage!(),
    };

    layout::generate(core_area, l3_area, cores)
}

fn find(database: &Database, table: &str, like: &str) -> Result<f64> {
    use sqlite::State;
    let mut statement = ok!(database.prepare(&format!(
        "SELECT name, value FROM `{}` WHERE name LIKE '{}' LIMIT 1;", table, like,
    )));
    Ok(match ok!(statement.step()) {
        State::Row => ok!(statement.column::<f64>(0 + 1)),
        _ => raise!("failed to find a required value in the table"),
    })
}

fn fail<E: Display>(error: E) -> ! {
    use std::io::{stderr, Write};
    stderr().write_fmt(format_args!("Error: {}.\n", error)).unwrap_or(());
    process::exit(1);
}

fn usage() -> ! {
    println!("{}", USAGE);
    process::exit(1);
}

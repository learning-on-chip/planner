#![cfg_attr(test, allow(dead_code))]

extern crate arguments;
extern crate sqlite;

use sqlite::Database;
use std::io;
use std::fmt::Display;
use std::path::Path;
use std::{env, process};

const CORE_LIKE: &'static str = "core%_area";
const L3_LIKE: &'static str = "l3%_area";

const USAGE: &'static str = "
Usage: smith [options]

Options:
    --database <path>        SQLite3 database (required).
    --table <name>           Table containing area estimates (required).
    --cores <number>         Number of cores (required).
    --format (3d-ice|svg)    Output format [default: 3d-ice].

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

mod format;
mod layout;

pub type Result<T> = std::result::Result<T, Box<Display>>;

fn main() {
    start().unwrap_or_else(|error| fail(&*error));
}

fn start() -> Result<()> {
    use format::Format;
    use layout::Layout;

    let arguments = ok!(arguments::parse(env::args()));

    if arguments.get::<bool>("help").unwrap_or(false) {
        usage();
    }

    let database = match arguments.get::<String>("database") {
        Some(ref database) => ok!(sqlite::open(&Path::new(database))),
        _ => raise!("a database filename is required"),
    };
    let (core_area, l3_area) = match arguments.get::<String>("table") {
        Some(ref table) => {
            (ok!(find(&database, table, CORE_LIKE)), ok!(find(&database, table, L3_LIKE)))
        },
        _ => raise!("a table name is required"),
    };
    let core_count = match arguments.get::<usize>("cores") {
        Some(core_count) if core_count > 0 => core_count,
        _ => raise!("a number of cores is required"),
    };

    let spec = layout::Spec {
        core_count: core_count,
        core_area: core_area,
        l3_area: l3_area,
    };

    let layout = layout::Tiles;
    let format = match &arguments.get::<String>("format").unwrap_or("3d-ice".to_string())[..] {
        "svg" => Box::new(format::SVG) as Box<Format>,
        "3d-ice" => Box::new(format::ThreeDICE) as Box<Format>,
        _ => raise!("the output format is unknown"),
    };

    format.print(&ok!(layout.construct(&spec)), &mut io::stdout())
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

#[allow(unused_must_use)]
fn fail<E: Display>(error: E) -> ! {
    use std::io::Write;
    let message = format!("Error: {}.\n", error);
    io::stderr().write_all(message.as_bytes());
    if message.contains("required") {
        io::stderr().write_all(format!("\n{}\n", USAGE.trim()).as_bytes());
    }
    process::exit(1);
}

fn usage() -> ! {
    println!("{}", USAGE.trim());
    process::exit(1);
}

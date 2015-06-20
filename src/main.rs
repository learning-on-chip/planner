#![cfg_attr(test, allow(dead_code))]

extern crate arguments;
extern crate sqlite;

use sqlite::Database;
use std::fmt::Display;

const CORE_LIKE: &'static str = "core%_area";
const L3_LIKE: &'static str = "l3%_area";

const USAGE: &'static str = "
Usage: planner [options]

Options:
    --database <path>        SQLite3 database (required).
    --table <name>           Table containing area estimates (required).
    --cores <number>         Number of cores (required).
    --format (3d-ice|svg)    Output format [default: 3d-ice].

    --help                   Display this message.
";

macro_rules! ok(
    ($result:expr) => (match $result {
        Ok(result) => result,
        Err(error) => raise!(error),
    });
);

macro_rules! raise(
    ($error:expr) => (return Err(Box::new($error)));
    ($($arg:tt)*) => (raise!(format!($($arg)*)));
);

mod format;
mod layout;

pub type Error = Box<Display>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    start().unwrap_or_else(|error| fail(error));
}

fn start() -> Result<()> {
    use format::Format;
    use layout::Layout;

    let arguments = ok!(arguments::parse(std::env::args()));

    if arguments.get::<bool>("help").unwrap_or(false) {
        help();
    }

    let database = match arguments.get::<String>("database") {
        Some(ref database) => {
            if std::fs::metadata(database).is_err() {
                raise!("the database does not exist");
            }
            ok!(sqlite::open(database))
        },
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

    format.print(&ok!(layout.construct(&spec)), &mut std::io::stdout())
}

fn find(database: &Database, table: &str, like: &str) -> Result<f64> {
    use sqlite::State;
    let mut statement = ok!(database.prepare(&format!(
        "SELECT name, value FROM `{}` WHERE name LIKE '{}' LIMIT 1;", table, like,
    )));
    Ok(match ok!(statement.step()) {
        State::Row => ok!(statement.read::<f64>(0 + 1)),
        _ => raise!("failed to find a required value in the table"),
    })
}

fn help() -> ! {
    println!("{}", USAGE.trim());
    std::process::exit(0);
}

fn fail(error: Error) -> ! {
    use std::io::{stderr, Write};
    stderr().write_all(format!("Error: {}.\n", &*error).as_bytes()).unwrap();
    std::process::exit(1);
}

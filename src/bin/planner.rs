extern crate arguments;
extern crate sql;
extern crate sqlite;
extern crate term;

#[macro_use]
extern crate planner;

use planner::layout::{self, Layout};
use planner::output::{self, Output};
use planner::{Error, Result};
use sqlite::Connection;

const USAGE: &'static str = "
Usage: planner [options]

Options:
    --database <path>        SQLite3 database (required).
    --table <name>           Table containing area estimates (required).
    --cores <number>         Number of cores (required).
    --output (3d-ice|svg)    Output format [default: 3d-ice].

    --help                   Display this message.
";

fn main() {
    start().unwrap_or_else(|error| fail(error));
}

fn start() -> Result<()> {
    let arguments = ok!(arguments::parse(std::env::args()));
    if arguments.get::<bool>("help").unwrap_or(false) {
        help();
    }

    let backend = match arguments.get::<String>("database") {
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
            (ok!(find(&backend, table, "core%")), ok!(find(&backend, table, "l3%")))
        },
        _ => raise!("a table name is required"),
    };
    let core_count = match arguments.get::<usize>("cores") {
        Some(core_count) if core_count > 0 => core_count,
        _ => raise!("a number of cores is required"),
    };

    let config = layout::Configuration {
        core_count: core_count,
        core_area: core_area,
        l3_area: l3_area,
    };

    let layout = layout::Tiles;
    let output = match &*arguments.get::<String>("output").unwrap_or("3d-ice".to_string()) {
        "svg" => Box::new(output::SVG) as Box<Output>,
        "3d-ice" => Box::new(output::ThreeDICE) as Box<Output>,
        _ => raise!("the output format is unknown"),
    };

    output.write(&ok!(layout.construct(&config)), &mut std::io::stdout())
}

fn find(backend: &Connection, table: &str, like: &str) -> Result<f64> {
    use sql::prelude::*;

    let statement = select_from(table).columns(&["name", "area"])
                                      .so_that(column("name").like(like)).limit(1);
    let mut cursor = ok!(backend.prepare(ok!(statement.compile()))).cursor();
    if let Some(row) = ok!(cursor.next()) {
        if let Some(value) = row[1].as_float() {
            return Ok(value);
        }
    }
    raise!("failed to find a required value in the table");
}

fn help() -> ! {
    println!("{}", USAGE.trim());
    std::process::exit(0);
}

#[allow(unused_must_use)]
fn fail(error: Error) -> ! {
    use std::io::{stderr, Write};
    if let Some(mut output) = term::stderr() {
        output.fg(term::color::RED);
        output.write_all(format!("Error: {}.\n", error).as_bytes());
    }
    std::process::exit(1);
}

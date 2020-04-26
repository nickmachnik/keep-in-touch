use chrono::{DateTime, TimeZone, Utc};
use clap::ArgMatches;
use std::fs::read_to_string;
use std::path::Path;

use crate::parse::parse_date;
use crate::table::{Entry, Table};
use crate::TABLE_LOC;

fn get_interval(raw: &str) -> usize {
    match raw.parse() {
        Ok(num) => num,
        Err(e) => {
            error!(
                "Parsing the interval field failed: {:?}. Please enter an integer.",
                e
            );
            std::process::exit(exitcode::USAGE);
        }
    }
}

fn get_date(raw: &str) -> DateTime<Utc> {
    match parse_date(raw) {
        Ok(date) => date,
        Err(e) => {
            error!(
                "Parsing the date string failed: {:?}. Required format: YEAR-MONTH-DAY",
                e
            );
            std::process::exit(exitcode::USAGE);
        }
    }
}

pub fn add(args: ArgMatches) {
    let table_path = Path::new(TABLE_LOC);
    let c = args.subcommand_matches("add").unwrap();
    let name = c.value_of("name").unwrap();
    let interval = get_interval(c.value_of("interval").unwrap());
    let last_chat = get_date(c.value_of("last chat").unwrap());
    let mut data = if let Ok(json_file_str) = read_to_string(table_path) {
        Table::from_json(json_file_str)
    } else {
        Table::new()
    };

    if data
        .add_entry(Entry::new(name.to_string(), interval, last_chat))
        .is_err()
    {
        error!(
            "Name {:?} is already used.  \
            Please choose a different name or modify the \
            existing entry.",
            name
        );
        std::process::exit(exitcode::CANTCREAT);
    }

    data.to_json(table_path);
    info!("Added {:?}.", name);
}

pub fn remove(args: ArgMatches) {
    unimplemented!()
}

pub fn modify(args: ArgMatches) {
    unimplemented!()
}

pub fn view(args: ArgMatches) {
    unimplemented!()
}

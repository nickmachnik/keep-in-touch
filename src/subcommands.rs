use clap::ArgMatches;
use std::fs::read_to_string;
use std::path::Path;

use crate::parse::parse_date;
use crate::table::{Entry, Table};
use crate::TABLE_LOC;

pub fn add(args: ArgMatches) {
    let table_path = Path::new(TABLE_LOC);
    let c = args.subcommand_matches("add").unwrap();
    let name = c.value_of("name").unwrap();
    let interval = c.value_of("interval").unwrap().parse();
    if let Err(e) = interval {
        error!(
            "Parsing the interval field failed: {:?}. Please enter an integer.",
            e
        );
        std::process::exit(exitcode::USAGE);
    }
    let last_chat = parse_date(c.value_of("last chat").unwrap());
    if let Err(e) = &last_chat {
        error!(
            "Parsing the date string failed: {:?}. Required format: YEAR-MONTH-DAY",
            e
        );
        std::process::exit(exitcode::USAGE);
    }
    let mut data = if let Ok(json_file_str) = read_to_string(table_path) {
        Table::from_json(json_file_str)
    } else {
        Table::new()
    };

    if data
        .add_entry(Entry::new(
            name.to_string(),
            interval.unwrap(),
            last_chat.unwrap(),
        ))
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

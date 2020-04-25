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
    let interval = c.value_of("interval").unwrap().parse().unwrap();
    let last_chat = parse_date(c.value_of("last chat").unwrap());
    if let Err(e) = &last_chat {
        error!("Parsing the date string failed: {:?}", e);
    }
    let mut data = if let Ok(json_file_str) = read_to_string(table_path) {
        Table::from_json(json_file_str)
    } else {
        Table::new()
    };

    if data.entries.contains_key(name) {
        error!(
            "Name {:?} is already used.  \
            Please choose a different name or modify the \
            existing entry.",
            name
        );
    } else {
        data.add_entry(Entry::new(name.to_string(), interval, last_chat.unwrap()));
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

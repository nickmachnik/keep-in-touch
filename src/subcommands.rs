use clap::ArgMatches;
// use std::ffi::OsStr;
// use std::fs::create_dir_all;
use std::fs::read_to_string;
// use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::table::{Entry, Table};

pub fn add(args: ArgMatches) {
    let c = args.subcommand_matches("add").unwrap();
    let name = c.value_of("name").unwrap();
    let interval = c.value_of("interval").unwrap().parse().unwrap();
    let last_chat = c.value_of("last chat").unwrap();
    let mut data = if let Ok(json_file_str) = read_to_string(Path::new("table.json")) {
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
        data.add_entry(Entry::new(name.to_string(), interval, last_chat));
    }
    // info!("All done.");
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

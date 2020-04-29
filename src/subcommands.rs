use chrono::{DateTime, Utc};
use clap::ArgMatches;

use std::path::PathBuf;

use crate::parse::parse_date;
use crate::table::{Entry, Table};
use crate::TABLE_LOC;

fn get_table_path() -> PathBuf {
    let mut outpath = std::env::current_exe().unwrap();
    outpath.set_file_name(TABLE_LOC);
    outpath
}

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
    let table_path = get_table_path();
    let c = args.subcommand_matches("add").unwrap();
    let name = c.value_of("name").unwrap();
    let interval = get_interval(c.value_of("interval").unwrap());
    let last_chat = get_date(c.value_of("last chat").unwrap());
    let mut data = Table::from_json(&table_path).unwrap_or_else(|_| Table::new());
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
    data.to_json(&table_path);
    info!("Added {:?}.", name);
}

pub fn remove(args: ArgMatches) {
    let table_path = get_table_path();
    let data = Table::from_json(&table_path);
    if data.is_err() {
        error!("List file not found. Please add entries first.");
        std::process::exit(exitcode::USAGE);
    }
    let c = args.subcommand_matches("remove").unwrap();
    let name = c.value_of("name").unwrap();
    let mut data = data.unwrap();
    if data.remove_entry(name.to_string()).is_err() {
        error!("Name {:?} is not in the list.", name);
        std::process::exit(exitcode::USAGE);
    }
    data.to_json(&table_path);
    info!("Removed {:?}.", name);
}

pub fn modify(args: ArgMatches) {
    let table_path = get_table_path();
    let data = Table::from_json(&table_path);
    if data.is_err() {
        error!("List file not found. Please add entries first.");
        std::process::exit(exitcode::USAGE);
    }
    let data = &mut data.unwrap();
    let c = args.subcommand_matches("modify").unwrap();
    let name = c.value_of("name").unwrap();
    let entry = data.entries.get_mut(name);
    if entry.is_none() {
        error!("Name {:?} is not in the list.", name);
        std::process::exit(exitcode::USAGE);
    }
    let entry = entry.unwrap();
    let raw_new_val = c.value_of("new value").unwrap();
    match c.value_of("field").unwrap() {
        "name" => {
            let new_entry = Entry::new(raw_new_val.to_string(), entry.interval, entry.last_contact);
            data.add_entry(new_entry).unwrap();
            data.remove_entry(name.to_string()).unwrap();
        }
        "interval" => {
            entry.interval = get_interval(raw_new_val);
            entry.update_remaining_time();
        }
        "last" => {
            entry.last_contact = get_date(raw_new_val);
            entry.update_remaining_time();
        }
        _ => {
            error!("Invalid field id. Use of 'name', 'interval', 'last'.");
            std::process::exit(exitcode::USAGE);
        }
    }
    data.to_json(&table_path);
    info!("Modified {:?}.", name);
}

pub fn view(_args: ArgMatches) {
    let table_path = get_table_path();
    let data = Table::from_json(&table_path);
    if data.is_err() {
        error!("List file not found. Please add entries before viewing.");
        std::process::exit(exitcode::USAGE);
    }
    let mut data = data.unwrap();
    data.update_entries_par();
    data.print_by_remaining_time();
}

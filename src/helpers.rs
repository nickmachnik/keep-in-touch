//! Small helper functions that perform often used operations.

use chrono::prelude::DateTime;
use chrono::prelude::Utc;
use log::{error, info};
use regex::Regex;
use std::fs::{read_to_string, File};
use std::io;
use std::io::Write;
use std::path::PathBuf;

use crate::parse::parse_date;
use crate::table::Table;
use crate::COMPLETION_LOC;
use crate::TABLE_LOC;

pub fn get_table_path() -> PathBuf {
    let mut outpath = std::env::current_exe().unwrap();
    outpath.set_file_name(TABLE_LOC);
    outpath
}

pub fn get_interval(raw: &str) -> usize {
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

pub fn get_date(raw: &str) -> DateTime<Utc> {
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

/// This updates the names in the kit-complete.sh script.
pub fn update_autocomplete_names(table: &Table) -> Result<(), io::Error> {
    let mut compl_path = std::env::current_exe().unwrap();
    compl_path.set_file_name(COMPLETION_LOC);
    let data = read_to_string(&compl_path)?;
    let re = Regex::new(r#"-W "(?s)(.*)" -- "$namepos""#).unwrap();
    let names = table
        .entries
        .keys()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    let new_data = re.replace_all(&data, format!(r#"-W "{}" -- "$namepos""#, &names).as_str());
    let mut dst = File::create(&compl_path)?;
    dst.write_all(new_data.as_bytes())?;
    info!("Updated names for autocompletion.");
    Ok(())
}

// pub fn generate_autocomplete_script() -> Result<(), io::Error> {
//     let mut compl_path = std::env::current_exe().unwrap();
//     compl_path.set_file_name(COMPLETION_LOC);
//     let mut dst = File::create(&compl_path)?;
//     dst.write_all(r#"COMPREPLY=($(compgen -W "KIT_SAVED_NAMES" -- "$namepos"))"#.as_bytes())?;
//     Ok(())
// }

//! Small helper functions that perform often used operations.

use crate::parse::parse_date;
use crate::table::Table;
use crate::TABLE_LOC;
use chrono::prelude::DateTime;
use chrono::prelude::Utc;
use std::path::PathBuf;

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

/// This updates the names in the kit-complete.sh script
pub fn update_autocomplete_names(_table: &Table) {
    unimplemented!()
}

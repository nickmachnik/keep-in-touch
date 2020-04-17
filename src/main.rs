extern crate chrono;
extern crate clap;
extern crate colored;
extern crate env_logger;
extern crate hashbrown;
#[macro_use]
extern crate log;
extern crate rayon;
extern crate serde;

use chrono::Local;
use colored::Colorize;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

mod table;

fn main() {
    // log time stamp
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    println!("{}", Local::now().to_string().on_blue().red());
}

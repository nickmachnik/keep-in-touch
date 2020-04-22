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
use clap::{App, AppSettings, Arg, SubCommand};

use env_logger::Builder;
use log::LevelFilter;

use std::io::Write;

mod subcommands;
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

    let add = SubCommand::with_name("add").about("").arg(
        Arg::with_name("input")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("Path to input fasta file (uncompressed)"),
    );

    let args = App::new("kit")
        .version("0.1.0")
        .author("Nick Noel Machnik <nick.machnik@gmail.com>")
        .about("Command line organizer that helps you remember to call your friends.")
        .subcommand(add)
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();
}

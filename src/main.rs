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

mod parse;
mod subcommands;
mod table;

const TABLE_LOC: &str = "./table.json";

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

    let add = SubCommand::with_name("add")
        .about("Add a person to your list.")
        .arg(
            Arg::with_name("name")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("Name of the person you want to add."),
        )
        .arg(
            Arg::with_name("interval")
                .required(true)
                .takes_value(true)
                .index(2)
                .help("How regularly do you want to talk to the person (in days)?"),
        )
        .arg(
            Arg::with_name("last chat")
                .required(true)
                .takes_value(true)
                .index(3)
                .help(
                    "The date of the last chat with your friend. Either `now` or \
                in a year-month-day format, e.g. `2000-5-4`.",
                ),
        );

    let args = App::new("kit")
        .version("0.1.0")
        .author("Nick Noel Machnik <nick.machnik@gmail.com>")
        .about("Command line organizer that helps you remember to call your friends.")
        .subcommand(add)
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    match args.subcommand_name() {
        Some("add") => {
            subcommands::add(args);
        }
        Some("remove") => {
            subcommands::remove(args);
        }
        Some("modify") => {
            subcommands::modify(args);
        }
        Some("view") => {
            subcommands::view(args);
        }
        Some(other) => unimplemented!("{}", other),
        None => panic!("what is supposed to happen here"),
    }
}

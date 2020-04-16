extern crate chrono;
extern crate clap;
extern crate colored;
extern crate rayon;

use chrono::prelude::*;
use colored::Colorize;

mod table;

fn main() {
    println!("{}", Local::now().to_string().on_blue().red());
}

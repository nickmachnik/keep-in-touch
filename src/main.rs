use chrono::Local;
use clap::{crate_version, App, AppSettings, Arg, SubCommand};
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

mod helpers;
mod parse;
mod subcommands;
mod table;

const TABLE_LOC: &str = "table.json";
const COMPLETION_LOC: &str = "kit-completion.sh";

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

    let no_update = Arg::with_name("no-autocomplete-update")
        .long("no-update")
        .short("n")
        .global(true)
        .help(
            "Disable update of the bash autocomplete script which \
         is done by default whenver a name is added or changed.",
        );

    let justtalkedto = SubCommand::with_name("just-talked-to")
        .about("Set the `last` date of a person to `now`.")
        .arg(
            Arg::with_name("name")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("Name of the person you just talked to."),
        );

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

    let suspend = SubCommand::with_name("suspend")
        .about(
            "Suspend an entry. \
            Inactivates colored highlighting and shows the entry at \
            the bottom of the table when printed.",
        )
        .arg(
            Arg::with_name("name")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("Name of the person you want to suspend."),
        );

    let resume = SubCommand::with_name("resume")
        .about("Reactivates a suspended entry.")
        .arg(
            Arg::with_name("name")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("Name of the person you want to reactivate."),
        );

    let remove = SubCommand::with_name("remove")
        .about("Remove a person from your list.")
        .arg(
            Arg::with_name("name")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("Name of the person you want to remove."),
        );

    let modify = SubCommand::with_name("modify")
        .about("Modify an entry.")
        .arg(
            Arg::with_name("name")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("Name of the person you who's entry you want to modify."),
        )
        .arg(
            Arg::with_name("field")
                .required(true)
                .takes_value(true)
                .index(2)
                .help(
                    "The entry field you want to modify. \
                    One of 'name', 'interval', 'last'.",
                ),
        )
        .arg(
            Arg::with_name("new value")
                .required(true)
                .takes_value(true)
                .index(3)
                .help("The value you want to replace the existing value with."),
        );

    let view = SubCommand::with_name("view").about("View the list.");

    let view_active = SubCommand::with_name("view-active").about("View active entries.");

    let view_inactive = SubCommand::with_name("view-inactive").about("View suspended entries.");

    let update_autocompletion = SubCommand::with_name("update-autocompletion")
        .about("Update the entry names in the kit-completion.sh.");

    let args = App::new("kit")
        .version(crate_version!())
        .author("Nick Noel Machnik <nick.machnik@gmail.com>")
        .about("Command line organizer that helps you remember to call your friends.")
        .arg(no_update)
        .subcommand(add)
        .subcommand(remove)
        .subcommand(view)
        .subcommand(modify)
        .subcommand(justtalkedto)
        .subcommand(view_active)
        .subcommand(view_inactive)
        .subcommand(suspend)
        .subcommand(resume)
        .subcommand(update_autocompletion)
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
        Some("view-active") => {
            subcommands::view_active(args);
        }
        Some("view-inactive") => {
            subcommands::view_inactive(args);
        }
        Some("suspend") => {
            subcommands::suspend(args);
        }
        Some("resume") => {
            subcommands::resume(args);
        }
        Some("just-talked-to") => {
            subcommands::just_talked_to(args);
        }
        Some("update-autocompletion") => {
            subcommands::update_autocompletion(args);
        }
        Some(other) => unimplemented!("{}", other),
        None => panic!("what is supposed to happen here"),
    }
}

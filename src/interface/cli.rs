use clap::{App, AppSettings, Arg, SubCommand};
use crate::application::command::Command;

pub fn parse_command_line() -> Command {
    let matches = App::new("root")
        .bin_name("root")
        .version(crate_version!())
        .version_short("v")
        .about("roots e-book manager")
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("config")
                    .about("Shows the configuration")
                    .arg(Arg::with_name("path")
                         .short("p")
                         .long("path")
                         .conflicts_with("default")
                         .help("Display the configuration file path"))
                    .arg(Arg::with_name("default")
                         .short("d")
                         .long("default")
                         .conflicts_with("path")
                         .help("Display configuration defaults")))
        .subcommand(SubCommand::with_name("fields")
                    .about("Shows fields that can be used in queries"))
        .subcommand(SubCommand::with_name("import")
                    .about("Imports new e-books")
                    .usage(
                        "root import <path>

EXAMPLES:
    root import ~/Downloads/
       -> imports books from ~/Downloads/")
                    .arg(Arg::with_name("path")
                         .help("Path to directory containing e-books")
                         .required(true)))
        .subcommand(SubCommand::with_name("list")
                    .about("Queries the library")
                    .usage(
                        "root list [FLAGS]

EXAMPLES:
    root list author:forster
      -> All titles by Forster

    root list --author howards end
      -> All authors of matching titles

    root list --isbn
      -> All known titles with ISBNs")
                    .arg(Arg::with_name("author")
                         .short("a")
                         .long("author")
                         .help("Show a list of matching authors"))
                    .arg(Arg::with_name("isbn")
                         .short("i")
                         .long("isbn")
                         .help("Show the ISBN number of each title"))
                    .arg(Arg::with_name("table")
                         .short("t")
                         .long("table")
                         .help("Print the matches in a table")))
        .subcommand(SubCommand::with_name("update")
                    .about("Updates the library"))
        .get_matches();
    match matches.subcommand() {
        ("config", Some(config)) => {
            Command::Config {
                path: config.is_present("path"),
                default: config.is_present("default")
            }
        },
        ("fields", _) => {
            Command::Fields
        },
        ("import", Some(import)) => {
            Command::Import {
                path: import.value_of("path")
                    .map(|v| String::from(v))
                    .unwrap()
            }
        },
        ("list", Some(list)) => {
            Command::List {
                author: list.is_present("author"),
                isbn: list.is_present("isbn"),
                table: list.is_present("table")
            }
        },
        ("update", _) => {
            Command::Update
        }
        _ => unreachable!()
    }
}

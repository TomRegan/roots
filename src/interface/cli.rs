use crate::application::command::Command;
use crate::configuration::Configuration;
use clap::{App, AppSettings, Arg, SubCommand};
use database::query::{list_fields, list_titles};

pub struct Application {
    cfg: Configuration,
}

impl Application {
    pub fn new(cfg: Configuration) -> Self {
        Application {
            cfg,
        }
    }

    pub fn run(self) -> Result<(), ()> {
        let cmd = parse_command_line();
        handle_command(self.cfg, cmd)
    }
}

fn handle_command(cfg: Configuration, cmd: Command) -> Result<(), ()> {
    // todo --- import command
    match cmd {
        Command::Config { .. } => handle_config_command(cfg, cmd),
        Command::Fields => handle_fields_command(cfg, cmd),
        Command::List { .. } => handle_list_command(cfg, cmd),
        Command::Update => handle_update_command(cfg, cmd),
        _ => {
            println!("The application failed to complete, the reason is: unsupported command {:?}", cmd);
            Err(())
        },
    }
}

fn handle_config_command(cfg: Configuration, cmd: Command) -> Result<(), ()> {
    match cmd {
        Command::Config { path: true, default: false } => {
            println!("{}", cfg.get_source());
            Ok(())
        }
        Command::Config { path: false, default: true } => {
            println!("{}", &Configuration::default());
            Ok(())
        }
        Command::Config { path: false, default: false } => {
            println!("{}", cfg);
            Ok(())
        }
        _ => Err(())
    }
}

fn handle_fields_command(_cfg: Configuration, cmd: Command) -> Result<(), ()> {
    match cmd {
        Command::Fields => {
            let available_fields = list_fields();
            if available_fields.is_empty() {
                println!("No available fields, is roots initialised?")
            } else {
                for f in available_fields {
                    println!("{}", f);
                }
            }
            Ok(())
        }
        _ => Err(())
    }
}

fn handle_list_command(_cfg: Configuration, cmd: Command) -> Result<(), ()> {
    match cmd {
        Command::List { .. } => {
            let available_titles = list_titles();
            if available_titles.is_empty() {
                println!("No titles to list, is roots initialised?");
            } else {
                for t in available_titles {
                    println!("{}", t);
                }
            }
            Ok(())
        }
        _ => Err(())
    }
}

fn handle_update_command(_cfg: Configuration, cmd: Command) -> Result<(), ()> {
    match cmd {
        Command::Update => {
            // scan the filesystem, cross-reference the database and ensure they're consistent
            // use a 3p service to cross-ref metadata for existing titles
            // offer to resolve any inconsistencies
            let available_titles = list_titles();
            if available_titles.is_empty() {
                println!("No titles found, is roots initialised?");
            }
            Ok(())
        }
        _ => Err(())
    }
}

fn parse_command_line() -> Command {
    let matches = App::new("root")
        .bin_name("root")
        .version(crate_version!())
        .version_short("v")
        .about("roots e-book manager")
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("config")
                .about("Shows the configuration")
                .arg(
                    Arg::with_name("path")
                        .short("p")
                        .long("path")
                        .conflicts_with("default")
                        .help("Display the configuration file path"),
                )
                .arg(
                    Arg::with_name("default")
                        .short("d")
                        .long("default")
                        .conflicts_with("path")
                        .help("Display configuration defaults"),
                ),
        )
        .subcommand(
            SubCommand::with_name("fields").about("Shows fields that can be used in queries"),
        )
        .subcommand(
            SubCommand::with_name("import")
                .about("Imports new e-books")
                .usage(
                    "root import <path>

EXAMPLES:
    root import ~/Downloads/
       -> imports books from ~/Downloads/",
                )
                .arg(
                    Arg::with_name("path")
                        .help("Path to directory containing e-books")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Queries the library")
                .usage(
                    "root list [FLAGS]

EXAMPLES:
    root list author:forster
      -> All titles by Forster

    root list --author howards end
      -> All authors of matching titles

    root list --isbn
      -> All known titles with ISBNs",
                )
                .arg(
                    Arg::with_name("author")
                        .short("a")
                        .long("author")
                        .help("Show a list of matching authors"),
                )
                .arg(
                    Arg::with_name("isbn")
                        .short("i")
                        .long("isbn")
                        .help("Show the ISBN number of each title"),
                )
                .arg(
                    Arg::with_name("table")
                        .short("t")
                        .long("table")
                        .help("Print the matches in a table"),
                ),
        )
        .subcommand(SubCommand::with_name("update").about("Updates the library"))
        .get_matches();
    match matches.subcommand() {
        ("config", Some(config)) => Command::Config {
            path: config.is_present("path"),
            default: config.is_present("default"),
        },
        ("fields", _) => Command::Fields,
        ("import", Some(import)) => Command::Import {
            path: import.value_of("path").map(|v| String::from(v)).unwrap(),
        },
        ("list", Some(list)) => Command::List {
            author: list.is_present("author"),
            isbn: list.is_present("isbn"),
            table: list.is_present("table"),
        },
        ("update", _) => Command::Update,
        _ => unreachable!(),
    }
}

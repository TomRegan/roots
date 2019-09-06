#[macro_use]
extern crate clap;

mod command {
    #[derive(Debug)]
    pub enum Command {
        Config {
            path: bool,
            default: bool
        },
        Fields,
        Import {
            path: String
        },
        List {
            author: bool,
            isbn: bool,
            table: bool
        },
        Update
    }
}


mod interface {

    use clap::{App, AppSettings, Arg, SubCommand};
    use super::command::Command;

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
}

mod configuration {
    extern crate config;

    use config::{Config, Environment};
    use std::vec::Vec;
    use std::path::PathBuf;

    #[derive(Debug)]
    struct List {
        isbn: bool,
        table: bool
    }

    #[derive(Debug)]
    struct Isbndb {
        key: String,
        limit: u8
    }

    #[derive(Debug)]
    struct Import {
        hash: bool,
        relocate: bool,
        overwrite: bool,
        prune: bool,
        replacements: Vec<String>
    }

    #[derive(Debug)]
    pub struct Configuration {
        debug: bool,
        directory: PathBuf,
        library: PathBuf,
        import: Import,
        list: List,
        isbndb: Option<Isbndb>
    }

    pub fn configuration() -> Configuration {
        let mut backing = Config::default();
        backing.merge(Environment::with_prefix("ROOTS")
                      .separator("_")).unwrap();
        Configuration {
                debug: backing.get_bool("debug").ok().unwrap_or(false),
                directory:
                PathBuf::from(backing.get_str("directory")
                              .unwrap_or("~/Books".to_string())),
                library:
                PathBuf::from(backing.get_str("library")
                              .unwrap_or("library.db".to_string())),
                import: Import {
                    hash: backing.get_bool("import.hash")
                        .unwrap_or(false),
                    relocate: backing.get_bool("import.move")
                        .unwrap_or(false),
                    overwrite: backing.get_bool("import.overwrite")
                        .unwrap_or(false),
                    prune: backing.get_bool("import.prune")
                        .unwrap_or(false),
                    replacements: vec![
                        String::from("[<>:\"?*|]"),
                        String::from("[/]"),
                        String::from("[\u{00}-\u{1f}]")
                    ]
                },
                list: List {
                    isbn: backing.get_bool("list.isbn").unwrap_or(false),
                    table: backing.get_bool("list.table").unwrap_or(false)
                },
                isbndb: None
            }
    }
}

fn main() {
    let env = configuration::configuration();
    let cmd = interface::parse_command_line();
    match cmd {
        command::Command::Config{path, ..} =>
            if path {
                println!("pooping today")
            } else {
                println!("not pooping today")
            }
        _ => println!("no poop ever")
    }
    println!("{:?}", cmd);
    println!("{:?}", env)
}

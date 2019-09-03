#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let _yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(_yaml).get_matches();
    if let Some(_) = matches.subcommand_matches("config") {
        let config = configuration::configuration();
        println!("{:?}", config);
    }
}

mod configuration {
    extern crate config;

    use config::{Config, Environment};
    use std::vec::Vec;
    use std::path::{Path, PathBuf};

    #[derive(Debug)]
    pub struct List {
        isbn: bool,
        table: bool
    }

    #[derive(Debug)]
    pub struct Isbndb {
        key: String,
        limit: u8
    }

    #[derive(Debug)]
    pub struct Import {
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

    impl Configuration {
        fn new(backing: Config) -> Configuration {
            Configuration {
                debug: backing.get_bool("debug").ok().unwrap_or(false),
                directory:
                PathBuf::from(backing.get_str("directory")
                              .unwrap_or("~/Books".to_string())),
                library:
                PathBuf::from(backing.get_str("library")
                              .unwrap_or("library.db".to_string())),
                import: Import {
                    hash: backing.get_bool("input.hash")
                        .unwrap_or(false),
                    relocate: backing.get_bool("input.move")
                        .unwrap_or(false),
                    overwrite: backing.get_bool("input.overwrite")
                        .unwrap_or(false),
                    prune: backing.get_bool("input.prune")
                        .unwrap_or(false),
                    replacements: vec![
                        String::from("[<>:\"?*|]"),
                        String::from("[/]"),
                        String::from("[\u{00}-\u{1f}]")
                    ]
                },
                list: List {
                    isbn: false,
                    table: false
                },
                isbndb: None
            }
        }

        pub fn debug(&self) -> bool {
            self.debug
        }

        pub fn library(&self) -> &Path {
            self.library.as_path()
        }
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

extern crate config;

use std::collections::HashMap;
use std::path::PathBuf;

use config::{Config, Environment};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct List {
    isbn: bool,
    table: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Isbndb {
    key: String,
    limit: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct Import {
    hash: bool,
    relocate: bool,
    overwrite: bool,
    prune: bool,
    replacements: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    debug: bool,
    directory: PathBuf,
    library: PathBuf,
    import: Import,
    list: List,
    isbndb: Option<Isbndb>,
}

pub fn load_configuration() -> Configuration {
    let mut backing = Config::default();
    backing
        .merge(Environment::with_prefix("ROOTS").separator("_"))
        .unwrap();
    Configuration {
        debug: backing.get_bool("debug").ok().unwrap_or(false),
        directory: PathBuf::from(
            backing
                .get_str("directory")
                .unwrap_or("~/Books".to_string()),
        ),
        library: PathBuf::from(
            backing
                .get_str("library")
                .unwrap_or("library.db".to_string()),
        ),
        import: Import {
            hash: backing.get_bool("import.hash").unwrap_or(false),
            relocate: backing.get_bool("import.move").unwrap_or(false),
            overwrite: backing.get_bool("import.overwrite").unwrap_or(false),
            prune: backing.get_bool("import.prune").unwrap_or(false),
            replacements: [
                (r#"[<>:"\?\*\|/]"#.to_string(), r#"_"#.to_string()),
                ("[\u{00}-\u{1f}]".to_string(), "".to_string()),
                (r#"\.$"#.to_string(), r#"_"#.to_string()),
                (r#"\s+$"#.to_string(), r#""#.to_string()),
                (r#"^\."#.to_string(), r#"_"#.to_string()),
            ]
            .iter()
            .cloned()
            .collect(),
        },
        list: List {
            isbn: backing.get_bool("list.isbn").unwrap_or(false),
            table: backing.get_bool("list.table").unwrap_or(false),
        },
        isbndb: None,
    }
}

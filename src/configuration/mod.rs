extern crate config;

use std::{env, path::PathBuf};
use std::collections::HashMap;

use config::{Config, Environment, File};
use maplit::hashmap;
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

impl Configuration {
    pub fn new() -> Configuration {
        load().try_into().unwrap()
    }
}

fn replacements() -> HashMap<String, String> {
    hashmap!{
        r#"[<>:"\?\*\|/]"#.to_string() => r#"_"#.to_string(),
        "[\u{00}-\u{1f}]".to_string() => "".to_string(),
        r#"\.$"#.to_string() => r#"_"#.to_string(),
        r#"\s+$"#.to_string() => r#""#.to_string(),
        r#"^\."#.to_string() => r#"_"#.to_string(),
    }
}

fn load() -> Config {
    let home = env::var("HOME").unwrap_or("./".into());
    Config::default()
        .set_default("debug", false).unwrap()
        .set_default("directory", "~/Books".to_string()).unwrap()
        .set_default("library", "library.db".to_string()).unwrap()
        .set_default("import.hash", false).unwrap()
        .set_default("import.relocate", false).unwrap()
        .set_default("import.overwrite", false).unwrap()
        .set_default("import.prune", false).unwrap()
        .set_default("import.replacements", replacements()).unwrap()
        .set_default("list.isbn", false).unwrap()
        .set_default("list.table", false).unwrap()
        .merge(File::with_name(&format!("{}/.config/roots/default", home)).required(false)).unwrap()
        .merge(Environment::with_prefix("ROOTS").separator("_")).unwrap()
        .to_owned()
}

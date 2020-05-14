extern crate config;

use std::{env, path::{Path, PathBuf}};
use std::collections::HashMap;
use std::fmt::{Display, Result, Formatter};

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
    #[serde(rename = "move")]
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
    #[serde(skip_serializing)]
    source: String,
}

impl Configuration {

    pub fn new() -> Configuration {
        load().try_into().unwrap()
    }

    pub fn get_source(self) -> String {
        self.source
    }
}

impl Default for Configuration {
    fn default() -> Self {
        defaults().try_into().unwrap()
    }
}

impl Display for Configuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}\n", serde_yaml::to_string(self).unwrap())
    }
}

fn default_replacements() -> HashMap<String, String> {
    hashmap!{
        r#"[<>:"\?\*\|/]"#.to_string() => r#"_"#.to_string(),
        "[\u{00}-\u{1f}]".to_string() => "".to_string(),
        r#"\.$"#.to_string() => r#"_"#.to_string(),
        r#"\s+$"#.to_string() => r#""#.to_string(),
        r#"^\."#.to_string() => r#"_"#.to_string(),
    }
}

fn defaults() -> Config {
    let user_config_path = user_config_path();
    Config::default()
        .set_default("debug", false).unwrap()
        .set_default("directory", "~/Books".to_string()).unwrap()
        .set_default("library", "library.db".to_string()).unwrap()
        .set_default("import.hash", false).unwrap()
        .set_default("import.move", false).unwrap()
        .set_default("import.overwrite", false).unwrap()
        .set_default("import.prune", false).unwrap()
        .set_default("import.replacements", default_replacements()).unwrap()
        .set_default("list.isbn", false).unwrap()
        .set_default("list.table", false).unwrap()
        .set("source", resolve_source(&user_config_path)).unwrap()
        .to_owned()
}

fn load() -> Config {
    let user_config_path = user_config_path();
    defaults()
        .merge(File::from(user_config_path.as_path()).required(false)).unwrap()
        .merge(Environment::with_prefix("ROOTS").separator("_")).unwrap()
        .to_owned()
}

fn user_config_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or("./".to_string());
    PathBuf::new().join(&home).join(".config/roots/default")
}

fn resolve_source(path: &Path) -> String {
    if path.with_extension("yml").is_file() {
        path.with_extension("yml")
            .as_path()
            .to_string_lossy()
            .to_string()
    } else {
        "Default configuration".to_string()
    }
}

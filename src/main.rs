#[macro_use]
extern crate clap;
extern crate config;
extern crate core;
extern crate maplit;
extern crate regex;
extern crate serde;
extern crate serde_yaml;

use serde_yaml::to_string as yaml_from_struct;

use application::command::Command;
use configuration::Configuration;
use database::query::{list_fields, list_titles};
use interface::cli::parse_command_line;

mod application;
mod configuration;
mod database;
mod interface;

fn main() {
    let cfg = Configuration::new();
    let cmd = parse_command_line();
    match cmd {
        Command::Config { path, .. } => {
            if path {
                println!("{}", cfg.get_source())
            } else {
                match yaml_from_struct(&cfg) {
                    Ok(s) => println!("{}\n", s),
                    Err(e) => println!("uhoh {:?}", e),
                }
            }
        }
        Command::Fields => {
            let available_fields = list_fields();
            if available_fields.is_empty() {
                println!("No available fields, is roots initialised?")
            } else {
                for f in available_fields {
                    println!("{}", f);
                }
            }
        }
        Command::List { .. } => {
            let available_titles = list_titles();
            if available_titles.is_empty() {
                println!("No titles to list, is roots initialised?");
            } else {
                for t in available_titles {
                    println!("{}", t);
                }
            }
        }
        Command::Update => {
            // scan the filesystem, cross-reference the database and ensure they're consistent
            // use a 3p service to cross-ref metadata for existing titles
            // offer to resolve any inconsistencies
            let available_titles = list_titles();
            if available_titles.is_empty() {
                println!("No titles found, is roots initialised?");
            }
        }
        _ => unreachable!(),
    }
}

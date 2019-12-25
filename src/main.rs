#[macro_use]
extern crate clap;
extern crate config;
extern crate regex;
extern crate serde;
extern crate serde_yaml;

mod application;
mod interface;
pub mod configuration;

use interface::cli;
use application::command::Command;
use serde_yaml::to_string as yaml_from_struct;

fn main() {
    let cfg = configuration::configuration();
    let cmd = cli::parse_command_line();
    match cmd {
        Command::Config{path, ..} =>
            if path {
                println!("Default configuration")
            } else {
                match yaml_from_struct(&cfg) {
                    Ok(s) => println!("{}\n", s),
                    Err(e) => println!("uhoh {:?}", e)
                }
            }
        _ => unreachable!()
    }
}

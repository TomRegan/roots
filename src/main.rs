#[macro_use]
extern crate clap;
extern crate chrono;
extern crate config;
extern crate core;
extern crate epub;
extern crate maplit;
extern crate mobi;
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate serde_yaml;
extern crate serde_json;
extern crate url;


use configuration::Configuration;
use interface::cli;

mod application;
mod configuration;
mod database;
mod filesystem;
mod interface;
mod internet;

fn main() -> Result<(), ()> {
    let cfg = Configuration::new();
    let app = cli::Application::new(cfg);
    app.run()
}

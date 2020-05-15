#[macro_use]
extern crate clap;
extern crate config;
extern crate core;
extern crate maplit;
extern crate regex;
extern crate serde;
extern crate serde_yaml;

use configuration::Configuration;
use interface::cli;

mod application;
mod configuration;
mod database;
mod interface;

fn main() {
    let cfg = Configuration::new();
    let app = cli::Application::new(cfg);
    app.run().ok();
}

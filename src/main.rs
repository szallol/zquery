#[macro_use]
extern crate error_chain;

extern crate env_logger;
extern crate log;

extern crate clap;
use clap::{App, Arg};

mod errors;
mod manager;
mod source;
mod sqlite;
mod xml;
mod zquery;
pub use errors::*;
pub use zquery::ZQuery;

fn main() {
    env_logger::init();

    let matches = App::new("zq")
        .version("0.1.0")
        .about("command line query tool(SQL) for different data sources and destinations")
        .author("Szallo L. <szallol@gmail.com>")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("INPUT")
                .help("Input source")
                .multiple(true)
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    ZQuery::new(matches).run();
}
#[macro_use] extern crate error_chain;

extern crate log;
extern crate env_logger;

extern crate clap;
use clap::{Arg, App};

mod errors;
mod manager;
mod zquery;
mod source;
mod sqlite;
mod xml;
pub use zquery::ZQuery;
pub use errors::*;

fn main() {
 env_logger::init();

 let matches = App::new("zq")
     .version("0.1.0")
     .about("command line query tool(SQL) for different data sources and destinations")
     .author("Szallo L. <szallol@gmail.com>")
     .arg(Arg::with_name("input")
         .short("i")
         .long("input")
         .value_name("INPUT")
         .help("Input source")
         .multiple(true)
         .required(true)
         .takes_value(true))
     .get_matches();

 ZQuery::new(matches).run();
}


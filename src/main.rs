#[macro_use]
extern crate failure;

use clap::{App, Arg};

mod errors;
mod manager;
mod source;
mod zquery;

pub use errors::*;
use zquery::ZQuery;

fn main() -> Result<()> {
    env_logger::init();

    let matches = App::new("zq")
        .version("0.1.0")
        .about("query tool(SQL) for different data sources and destinations")
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

    if let Err(ref e) = ZQuery::new(matches)?.run() {
        println!("{}", e);
    }

    Ok(())
}

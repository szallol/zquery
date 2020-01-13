#[macro_use]
extern crate failure;

use clap::{App, Arg};

mod errors;
mod source;
mod zquery;

pub use errors::*;
use zquery::ZQuery;

fn main() -> Result<()> {
    let _logger = simple_logger::init().map_err(|_|
                                            println!("Failed to init log system.")
    );

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

    ZQuery::new(matches).unwrap().run().map_err (|e : ZqError| {
        println!("{}", e);
    });

    Ok(())
}

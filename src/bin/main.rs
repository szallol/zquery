extern crate clap;
extern crate zquery;

use clap::{Arg, App};
use zquery::ZQuery;

fn main() {
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

 match ZQuery::new(matches) {
  Ok(zquery) => {
   zquery.run();
  },
  Err(_) => {}
 }
}


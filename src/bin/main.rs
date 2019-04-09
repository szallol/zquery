extern crate clap;
extern crate zquery;

use clap::App;
use zquery::ZQuery;

fn main() {
 App::new("zq")
       .version("0.1.0")
      .about("command line query tool(SQL) for differrent data sources and destinations")
       .author("Szallo Laszlo")
       .get_matches();
}

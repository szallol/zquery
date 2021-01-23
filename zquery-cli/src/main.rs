use simple_logger::SimpleLogger;
use anyhow::Result;
use clap::{App, Arg};
use zquery_core::ZQuery;
    
fn main() -> Result<()> {
     SimpleLogger::new().init().unwrap();

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

    let zq = ZQuery{};

    //match ZQuery::new(matches).unwrap().run() {
        //Err(e) => println!("{}", e),
        //Ok(_) => {}
    //};

    Ok(())
}

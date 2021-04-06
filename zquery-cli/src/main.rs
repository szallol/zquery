use simple_logger::SimpleLogger;
use anyhow::Result;
use std::path::{Path, PathBuf};
use clap::{App, Arg};
use zquery_core::Zq;
use zquery_core::config::{InputConfig, OutputConfig, Config};
    
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
                .takes_value(true)
                )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT")
                .help("Output destination")
                .multiple(false)
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    let mut export_paths = PathBuf::new();
    export_paths.push("dummy.db");
    
    let input_strs = matches.values_of("input").unwrap();
    let config_inputs = InputConfig::from_strs(input_strs.collect())?; 

    let output_str = matches.value_of("input").unwrap(); 
    let config_output = OutputConfig::from_str(output_str)?;

    let config = Config::new(config_inputs, config_output);

    let zq = Zq::new(config)?;
    zq.import(Path::new("dummy"));
    zq.execute_query(String::from("SELECT sqlite_version()"))?
    .export(&export_paths);

    //match ZQuery::new(matches).unwrap().run() {
        //Err(e) => println!("{}", e),
        //Ok(_) => {}
    //};

    Ok(())
}

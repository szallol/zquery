use log::*;
use url::Url;

use std::io;
use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

use crate::errors::*;
use crate::manager::{ZqColumn, ZqCore, ZqTable};
use crate::source::ZqSource;

pub struct ZqXml {
    url: Url,
}

impl ZqXml {
    pub fn new(url: Url) -> Self {
        info!("new XML input: {}", url.to_string());
        ZqXml { url }
    }
}

impl ZqSource for ZqXml {
    fn import(&self, core: &mut dyn ZqCore) -> Result<()> {
        let mut tmp_columns = vec![
            ZqColumn::new("coll1", "INTEGER")?,
            ZqColumn::new("coll2", "INTEGER")?,
        ];

        core.create_table(&ZqTable::new("tmpTable", tmp_columns)?)?;

        let file = self.url.host_str().unwrap();
        let file = format!("{}{}", file, self.url.path());
        let file = File::open(file).map_err( |e : io::Error |
            ZqError::IoError {
                message: String::from("Failed to open database"),
                backtrace: failure::Backtrace::new(),
                cause: e
            }
        )?;

        let file = BufReader::new(file);

        let parser = EventReader::new(file);
        let mut _depth = 0;
        let mut ident = String::new();

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement{name, ..}) => {
                    info!("{}{}", ident, name);
                    ident.push('\t');
                }
                Ok(XmlEvent::EndElement{name, ..}) => {
                    ident.pop();
                    info!("{}{}\n\n", ident, name);
                }
                Ok(XmlEvent::Characters(data)) => {
                    info!("{}{}", ident, data);
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
        info!("xml imported from: {:?}, {:?}", self.url.host_str(), self.url.path());
//        info!("xml imported from: {:?}", parser);
        Ok(())
    }
}

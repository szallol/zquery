extern crate env_logger;
extern crate log;
extern crate xml;

use log::*;
use url::Url;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

use rusqlite::NO_PARAMS;

use crate::errors::*;
use crate::source::ZqSource;
use crate::manager::ZqCore;

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
    fn import(&self, core: &mut ZqCore) -> Result<()> {

        core.execute_query("CREATE TABLE person (
            id              INTEGER PRIMARY KEY,
        name            TEXT NOT NULL,
        time_created    TEXT NOT NULL,
        data            BLOB
        )",
        NO_PARAMS)?;

        info!("xml imported from: {}", self.url.to_string());
        Ok(())
    }
}

extern crate log;
extern crate env_logger;

use log::*;
use rusqlite::types::ToSql;
use rusqlite::{Connection, Result, NO_PARAMS};

use url::{Url};
//use crate::source::ZqSourceError;

use crate::source::ZqSource;

pub struct ZqSqlite {
    url : Url
}

impl ZqSqlite {
    pub fn new(url : Url) -> Self {
        info!("new SQLite from: {}", url.to_string());
        ZqSqlite{url}
    }
}

impl ZqSource for ZqSqlite {
    fn import (&self) {
        info!("SQLite imported from: {}", self.url.to_string());
    }
}
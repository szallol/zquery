extern crate log;
extern crate env_logger;

use log::*;
use rusqlite::types::ToSql;
use rusqlite::{Connection, Result, NO_PARAMS, Error, ErrorCode};

use url::{Url, ParseError};
use crate::source::ZqSource;

use std::path::Path;

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
    fn import (&self)
    {
        let mut db_file = self.url.host_str().unwrap().to_owned();
        let path = self.url.path();
        db_file.push_str(path);
        let db_file = db_file.to_string();
        let db_file = Path::new(&db_file);

//        let conn = Connection::open(db_file).unwrap_err();
        info!("SQLite imported from: {:?}", db_file);
    }
}
extern crate env_logger;
extern crate log;

pub use crate::errors::*;

use log::*;
use rusqlite::{Connection};

use crate::source::ZqSource;
use crate::manager::ZqCore;
use url::Url;
use std::path::Path;

pub struct ZqSqlite {
    url: Url,
}

impl ZqSqlite {
    pub fn new(url: Url) -> Self {
        info!("new SQLite from: {}", url.to_string());
        ZqSqlite { url }
    }
}

impl ZqSource for ZqSqlite {
    fn import(&self, _core : &mut ZqCore) -> Result<()> {
        let mut db_file = self.url.host_str().unwrap().to_owned();
        let path = self.url.path();
        db_file.push_str(path);
        let db_file = db_file.to_string();
        let db_file = Path::new(&db_file);

        let _conn = Connection::open(db_file).map_err(ZqError::RuSqlite)?;
        info!("SQLite imported from: {:?}", db_file);
        Ok(())
    }
}

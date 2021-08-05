use crate::errors::*;
use crate::source::ZqSource;
// use crate::zquery::ZqCore;

//use rusqlite::types::ToSql;
use rusqlite::Connection;

use log::*;
use std::path::Path;
use url::Url;
use crate::Zq;

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
    fn import(&self, _core: &Zq) -> Result<()> {
        let mut db_file = self.url.host_str().unwrap().to_owned();
        let path = self.url.path();
        db_file.push_str(path);
        let db_file = db_file.to_string();
        let db_file = Path::new(&db_file);

        let _conn =
            Connection::open(db_file).map_err(ZqError::Db)?;

        info!("SQLite imported from: {:?}", db_file);
        Ok(())
    }
}

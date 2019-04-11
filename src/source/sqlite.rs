//use rusqlite::types::ToSql;
//use rusqlite::{Connection, Result, NO_PARAMS};

use url::{Url};
//use crate::source::ZqSourceError;

use crate::source::ZqSource;

pub struct ZqSqlite {
    url : Url
}

impl ZqSqlite {
    pub fn new(url : Url) -> Self {
        println!("new ZqSqlite: {}", url.to_string());
        ZqSqlite{url}
    }
}

impl ZqSource for ZqSqlite {

}
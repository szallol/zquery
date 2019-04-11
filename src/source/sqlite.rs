use rusqlite::types::ToSql;
use rusqlite::{Connection, Result, NO_PARAMS};

use url::{Url, ParseError};
use crate::source::ZqSourceError;

pub struct ZqSqlite {

}

impl ZqSqlite {
    pub fn new() -> Self {
        ZqSqlite{}
    }
}
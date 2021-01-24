use std::sync::mpsc::{channel, Sender, Receiver};
use rusqlite::{Connection, NO_PARAMS};
use std::path::Path;
use super::errors::ZqError;

pub struct DbMsg{}
type DbSender = Sender<DbMsg>;

pub struct Db {
    _db_conn : Connection,
    _rx : Receiver<DbMsg>,
    _tx : DbSender,
}


impl Db {
    pub fn new() -> Result<Db, ZqError> {
        let conn = Connection::open(Path::new("tmpdb.db"))?;

        let (tx, rx) = channel::<DbMsg>();

        Ok(Db{_db_conn : conn, _rx : rx, _tx : tx})
    }

    pub fn _get_sender(&self) -> DbSender {
       self._tx.clone() 
    }

    pub fn _db_version(&self) -> Result<String, ZqError> { 
        let version = self._db_conn.query_row("SELECT sqlite_version()", NO_PARAMS, |row| row.get(0))?;
        Ok(version)
    }
}
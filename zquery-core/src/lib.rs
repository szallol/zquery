// use std::thread;
// use std::sync::mpsc::channel;
use std::path::Path;

mod db;
mod errors;

use db::Db;
use errors::ZqError;

pub struct Zq {
}

pub struct ZqIngest{
    db : Db,
}

pub struct ZqDone {
    _db : Db,
}

impl Zq {
    pub fn new() -> Result<ZqIngest, ZqError> {
        let db = Db::new()?;
        Ok(ZqIngest{db})
    }
}

impl ZqIngest {
    pub fn import(&self,  _source_path : &Path) {

    }
    
    pub fn execute_query (self, _query : String) -> Result<ZqDone, ZqError> {
        Ok(ZqDone{_db : self.db})
    }

}
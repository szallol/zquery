// use std::thread;
// use std::sync::mpsc::channel;
use std::path::{Path, PathBuf};

mod db;
mod errors;
pub mod config;

use db::Db;
use errors::ZqError;
use config::Config;

pub struct Zq {
}

pub struct ZqIngest{
    db : Db,
    config : Config,
}

pub struct ZqDone {
    db : Db,
    config : Config,
}

impl Zq {
    pub fn new(config: Config) -> Result<ZqIngest, ZqError> {
        let db = Db::new()?;
        Ok(ZqIngest{db, config})
    }
}

impl ZqIngest {
    pub fn import(&self,  _source_path : &Path) {

    }
    
    pub fn execute_query (self, _query : String) -> Result<ZqDone, ZqError> {
        Ok(ZqDone{db : self.db, config: self.config})
    }

}

impl ZqDone {
    pub fn export(self, _destinations : &PathBuf) {
        if let Some(output) = self.config.output() {
            println!("export done: {}", output);
        }

        self.db.stop_worker();
    } 
}

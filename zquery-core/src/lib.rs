// use std::thread;
// use std::sync::mpsc::channel;
use std::path::{Path, PathBuf};

mod db;
mod errors;
pub mod config;
pub mod source;

use db::Db;
use errors::ZqError;
use config::Config;
use source::ZqSource;

pub struct Zq {
    db : Db,
    config : Config,
    // inputs : Vec<Box<dyn ZqSource>>,
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
    pub fn new(config: Config) -> Result<Zq, ZqError> {
        let db = Db::new()?;
        Ok(Zq{db, config})
    }

    pub fn import(self) -> Result<ZqIngest, ZqError> {
        Ok(ZqIngest{db: self.db, config: self.config})
    }
    
}

impl ZqIngest {
    pub fn execute_query (self, query : String) -> Result<ZqDone, ZqError> {
        // self.db
        //     .execute(query, params)
        //     .map_err(|_| ZqError::QueryError {
        //         message: String::from("Failed to execute query"),
        //     })?;
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

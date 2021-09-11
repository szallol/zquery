// use std::thread;
// use std::sync::mpsc::channel;
// use std::path::{Path, PathBuf};
use std::path::PathBuf;

pub mod config;
mod db;
pub mod destination;
mod errors;
pub mod source;

use config::Config;
use db::Db;
use destination::ZqDestinationPool;
use errors::Result;
use source::ZqSourcePool;

pub struct Zq {
    db: Db,
    config: Config,
    _sources: Option<ZqSourcePool>,
    _destinations: Option<ZqDestinationPool>,
}

pub struct ZqIngest {
    zq: Zq,
}

pub struct ZqDone {
    zq: Zq,
}

impl Zq {
    pub fn new(config: Config) -> Result<Zq> {
        let db = Db::new()?;
        Ok(Zq {
            db,
            config,
            _sources: None,
            _destinations: None,
        })
    }

    pub fn import(self) -> Result<ZqIngest> {
        Ok(ZqIngest { zq: self })
    }
}

impl ZqIngest {
    pub fn execute_query(self, query : &str) -> Result<ZqDone> {
        self.zq.db
            .execute_query(query)?;

        Ok(ZqDone { zq: self.zq })
    }
}

impl ZqDone {
    pub fn export(self, _destinations: &PathBuf) -> Result<()> {
        if let Some(output) = self.zq.config.output() {
            println!("export done: {}", output);
        }

        println!("db version: {}", self.zq.db.db_version()?);
        self.zq.db.stop_worker();

        Ok(())
    }
}

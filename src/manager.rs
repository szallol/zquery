use url::Url;

use rusqlite::{Connection, ToSql, NO_PARAMS};

pub use crate::errors::*;

pub use crate::source::ZqSource;
pub use crate::sqlite::ZqSqlite;
pub use crate::xml::ZqXml;
use core::borrow::BorrowMut;

pub struct Manager {
    pub inputs: Vec<Box<dyn ZqSource>>,
    conn : rusqlite::Connection,
}

impl Manager {
    pub fn new() -> Result<Manager> {
        let inputs = Vec::new();
        let conn = Connection::open_in_memory().map_err(ZqError::RuSqlite)?;
        Ok(Manager { inputs, conn })
    }

    pub fn add_source(&mut self, source: &str) -> Result<&Manager> {

        let input_url = Url::parse(source).map_err(ZqError::UrlParse)?;
        match input_url.scheme() {
            "sqlite" => {
                let new_source = Box::new(ZqSqlite::new(input_url));
                new_source.import(self.borrow_mut())?;
                self.inputs.push(new_source);
            }
            "xml" => {
                let new_source = Box::new(ZqXml::new(input_url));
                new_source.import(self.borrow_mut())?;
                self.inputs.push(new_source);
            }
            _ => {}
        };
        Ok(self)
    }
}

pub trait ZqCore {
    fn execute_query(&self, query : &str) -> Result<()>;
}

impl ZqCore for Manager {
    fn execute_query(&self, query : &str) -> Result<()>
    {
        self.conn.execute(query, NO_PARAMS).map_err(ZqError::RuSqlite)?;
        Ok(())
    }
}

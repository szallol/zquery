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

pub struct ZqColumn {
    name : String,
    sqltype : String,
}

impl ZqColumn {
    pub fn new (name : String, sqltype : String) -> Result<ZqColumn> {
       Ok(ZqColumn{name, sqltype})
    }
}

pub struct ZqTable {
    pub name : String,
    pub columns: Vec<ZqColumn>,
}

impl ZqTable {
   pub fn new(name : String, columns : Vec<ZqColumn>)  -> Result<ZqTable> {
      Ok( ZqTable{name, columns})
   }

    pub fn name(self) -> Result<String> {
       Ok(self.name)
    }

    pub fn columns(self) -> Result<Vec<ZqColumn>> {
        Ok(self.columns)
    }
}

pub trait ZqCore {
    fn execute_query(&self, query : &str, params: &[(&dyn ToSql)]) -> Result<()>;
    fn create_table(&self, table : &ZqTable) -> Result<()>;
}

impl ZqCore for Manager {
    fn execute_query(&self, query : &str, params: &[(&dyn ToSql)]) -> Result<()>
    {
        self.conn.execute(query, params).map_err(ZqError::RuSqlite)?;
        Ok(())
    }

    fn create_table(&self, table : &ZqTable) -> Result<()> {
        let tableName = table.name()?;
        let query = format!("CREATE TABLE {}", tableName);
        table.columns().iter().for_each(|el| {

        });
//        self.conn.execute("CREATE TABLE ?1 (
//            id              INTEGER PRIMARY KEY,
//        name            TEXT NOT NULL,
//        time_created    TEXT NOT NULL,
//        data            BLOB
//        )",
//                          NO_PARAMS).unwrap(); //map_err(ZqError::RuSqlite)?;

        Ok(())
    }
}

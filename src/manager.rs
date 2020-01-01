
use url::Url;
use log::*;

use rusqlite::{Connection, ToSql, NO_PARAMS};

pub use crate::errors::*;

pub use crate::source::ZqSource;
pub use crate::source::sqlite::ZqSqlite;
pub use crate::source::xml::ZqXml;

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

pub struct ZqColumn<'a> {
    name : &'a str,
    sql_type: &'a str,
}

impl<'a> ZqColumn<'a> {
    pub fn new (name : &'a str, sqltype : &'a str) -> Result<ZqColumn<'a>> {
       Ok(ZqColumn{name, sql_type: sqltype })
    }

    #[allow(dead_code)]
    pub fn name(&self) -> Result<&'a str> {
        Ok(self.name)
    }

    #[allow(dead_code)]
    pub fn sql_type(&self) -> Result<&'a str> {
        Ok(self.sql_type)
    }
}

pub struct ZqTable<'a> {
     name : &'a str,
     columns: Vec<ZqColumn<'a>>,
}

impl<'a> ZqTable<'a> {
    pub fn new(name : &'a str, columns : Vec<ZqColumn<'a>>)  -> Result<ZqTable<'a>> {
        Ok( ZqTable{name, columns})
    }

    pub fn name(&self) -> Result<&'a str> {
       Ok(self.name)
    }
    pub fn columns(&self) -> Result<&Vec<ZqColumn>> {
        Ok(&self.columns)
    }
}

pub trait ZqCore {
    fn execute_query(&self, query : &str, params: &[&dyn ToSql]) -> Result<()>;
    fn create_table(&self, table : &ZqTable) -> Result<()>;
}

impl ZqCore for Manager {
    fn execute_query(&self, query : &str, params: &[&dyn ToSql]) -> Result<()>
    {
        self.conn.execute(query, params).map_err(ZqError::RuSqlite)?;
        Ok(())
    }

    fn create_table(&self, table : &ZqTable) -> Result<()> {
        let table_name = table.name()?;
        let mut query = format!("CREATE TABLE {}", table_name);
        let mut columns_query = String::from("(");
        if let Some ((last, elements)) = table.columns()?.split_last() {
            elements.into_iter().for_each(|column| {
                columns_query.push_str(&format!("{} {}, ", column.name, column.sql_type));
            });

            columns_query.push_str(&format!("{} {})", last.name, last.sql_type));
        }

        query.push_str(&columns_query);
        info!("{:?}", &query);
        self.conn.execute(&query, NO_PARAMS).unwrap();

        Ok(())
    }
}

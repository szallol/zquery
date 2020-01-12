use log::*;
use url::Url;
use failure::{Backtrace};

use rusqlite::{Connection, ToSql, NO_PARAMS};

pub use crate::errors::*;

pub use crate::source::sqlite::ZqSqlite;
pub use crate::source::xml::ZqXml;
pub use crate::source::ZqSource;

use core::borrow::BorrowMut;
use std::path::Path;

pub struct Manager {
    pub inputs: Vec<Box<dyn ZqSource>>,
    conn: rusqlite::Connection,
}

impl Manager {
    pub fn new() -> Result<Manager> {
        let inputs = Vec::new();
//        let conn = Connection::open_in_memory().map_err(ZqError::RuSqlite)?;
        let conn = Connection::open(Path::new("tmpdb.db")).map_err(|e : rusqlite::Error|
            ZqError::SqLiteError {
                message: String::from("Failed to open database"),
                backtrace: Backtrace::new(),
                cause: e
            })?;

        Ok(Manager { inputs, conn })
    }

    pub fn add_source(&mut self, source: &str) -> Result<&Manager> {
        let input_url = Url::parse(source).map_err(|_|
            ZqError::ParseError {message : String::from("Failed to parse source url")}
        )?;

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
    name: &'a str,
    sql_type: &'a str,
}

impl<'a> ZqColumn<'a> {
    pub fn new(name: &'a str, sqltype: &'a str) -> Result<ZqColumn<'a>> {
        Ok(ZqColumn {
            name,
            sql_type: sqltype,
        })
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
    name: &'a str,
    columns: Vec<ZqColumn<'a>>,
}

impl<'a> ZqTable<'a> {
    pub fn new(name: &'a str, columns: Vec<ZqColumn<'a>>) -> Result<ZqTable<'a>> {
        Ok(ZqTable { name, columns })
    }

    pub fn name(&self) -> Result<&'a str> {
        Ok(self.name)
    }
    pub fn columns(&self) -> Result<&Vec<ZqColumn>> {
        Ok(&self.columns)
    }
}

pub trait ZqCore {
    fn execute_query(&self, query: &str, params: &[&dyn ToSql]) -> Result<()>;
    fn create_table(&self, table: &ZqTable) -> Result<()>;
}

impl ZqCore for Manager {
    fn execute_query(&self, query: &str, params: &[&dyn ToSql]) -> Result<()> {
        self.conn
            .execute(query, params)
            .map_err(|_| {
                ZqError::QueryError {message : String::from("Failed to execute query")}
            })?;
        Ok(())
    }

    fn create_table(&self, table: &ZqTable) -> Result<()> {
        let table_name = table.name()?;

        let query = format!("DROP TABLE IF EXISTS {}", table_name);
        self.conn.execute(&query, NO_PARAMS).unwrap();

        let mut query = format!("CREATE TABLE {}", table_name);
        let mut columns_query = String::from("(");
        if let Some((last, elements)) = table.columns()?.split_last() {
            elements.into_iter().for_each(|column| {
                columns_query.push_str(&format!("{} {}, ", column.name, column.sql_type));
            });

            columns_query.push_str(&format!("{} {})", last.name, last.sql_type));
        }

        query.push_str(&columns_query);
        info!("{:?}", &query);

        self.conn.execute(&query, NO_PARAMS).map_err(|_| {
            ZqError::QueryError {message : String::from("Failed to execute query")}
        })?;

        Ok(())
    }
}

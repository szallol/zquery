use clap::ArgMatches;
use failure::Backtrace;
use log::*;
use rusqlite::{Connection, ToSql, NO_PARAMS};
use std::cell::RefCell;
use std::path::Path;
use url::Url;

pub mod column;
pub mod table;
pub mod sources;

use crate::errors::*;
use crate::source::{sqlite::ZqSqlite, xml::ZqXml, ZqSource};
use crate::zquery::table::*;

pub trait ZqCore {
    fn execute_query(&self, query: &str, params: &[&dyn ToSql]) -> Result<()>;
    fn create_table(&self, table: &ZqTable) -> Result<()>;
}

pub struct ZQuery<'a> {
    pub args: RefCell<ArgMatches<'a>>,
    pub inputs: RefCell<Vec<Box<dyn ZqSource>>>,
    conn: rusqlite::Connection,
}

impl<'a> ZQuery<'a> {
    pub fn new(args: ArgMatches<'a>) -> Result<Self> {
        let inputs = Vec::new();
        //        let conn = Connection::open_in_memory().map_err(ZqError::RuSqlite)?;
        let conn = Connection::open(Path::new("tmpdb.db")).map_err(|e: rusqlite::Error| {
            ZqError::SqLiteError {
                message: String::from("Failed to open database"),
                backtrace: Backtrace::new(),
                cause: e,
            }
        })?;

        let args = RefCell::new(args);
        let inputs = RefCell::new(inputs);
        Ok(ZQuery { args, inputs, conn })
    }

    pub fn run(&self) -> Result<()> {
        let args = self.args.borrow();
        let mut input_values = args.values_of("input").unwrap().into_iter();
        while let Some(input) = input_values.next() {
            self.add_source(input)?;
        }

        Ok(())
    }

    pub fn add_source(&self, input: &str) -> Result<()> {
        let input_url = Url::parse(input)
            .map_err(|_| ZqError::ParseError {
                message: String::from("Failed to parse source url"),
            })
            .unwrap();

        match input_url.scheme() {
            "sqlite" => {
                let new_source = Box::new(ZqSqlite::new(input_url));
                new_source.import(self)?;
                let mut inputs = self.inputs.borrow_mut();
                inputs.push(new_source);
            }
            "xml" => {
                let new_source = Box::new(ZqXml::new(input_url));
                new_source.import(self)?;
                let mut inputs = self.inputs.borrow_mut();
                inputs.push(new_source);
            }
            _ => {}
        };
        //                        Err(err) => {
        //                            error!("Failed to load input {} : {}", next_input, err);
        //                        }
        //                        _ => {
        //                        }
        //                    }
        //                };

        Ok(())
    }
}

impl ZqCore for ZQuery<'_> {
    fn execute_query(&self, query: &str, params: &[&dyn ToSql]) -> Result<()> {
        self.conn
            .execute(query, params)
            .map_err(|_| ZqError::QueryError {
                message: String::from("Failed to execute query"),
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
                columns_query.push_str(&format!(
                    "{} {}, ",
                    column.name().unwrap(),
                    column.sql_type().unwrap()
                ));
            });

            columns_query.push_str(&format!("{} {})", last.name()?, last.sql_type()?));
        }

        query.push_str(&columns_query);
        info!("{:?}", &query);

        self.conn
            .execute(&query, NO_PARAMS)
            .map_err(|_| ZqError::QueryError {
                message: String::from("Failed to execute query"),
            })?;

        Ok(())
    }
}

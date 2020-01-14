use clap::{ArgMatches, Values};
use core::borrow::BorrowMut;
use failure::Backtrace;
use log::*;
use rusqlite::{Connection, ToSql, NO_PARAMS};
use std::path::Path;
use url::Url;

pub mod column;
pub mod table;

use crate::errors::*;
use crate::source::{sqlite::ZqSqlite, xml::ZqXml, ZqSource};
use crate::zquery::table::*;

pub trait ZqCore {
    fn execute_query(&self, query: &str, params: &[&dyn ToSql]) -> Result<()>;
    fn create_table(&self, table: &ZqTable) -> Result<()>;
}

pub struct ZQuery<'a> {
    pub args: ArgMatches<'a>,
    pub inputs: Vec<Box<dyn ZqSource>>,
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

        Ok(ZQuery { args, inputs, conn })
    }

    pub fn run(&mut self) -> Result<()> {
        let input_values: Vec<_> =  self.args.values_of("input").unwrap().clone().collect();
        self.add_sources(input_values);

        Ok(())
    }

    pub fn add_sources(&mut self, sources: Vec<&str>) -> Result<()> {
        sources.into_iter().map(|source| {
            //            let input_url = Url::parse(source).map_err(|_|
            //                ZqError::ParseError {message : String::from("Failed to parse source url")}
            //            );
        });

        //            match input_url.scheme() {
        //                "sqlite" => {
        //                    let new_source = Box::new(ZqSqlite::new(input_url));
        //                    new_source.import(self.borrow_mut())?;
        //                    self.inputs.push(new_source);
        //                }
        //                "xml" => {
        //                    let new_source = Box::new(ZqXml::new(input_url));
        //                    new_source.import(self.borrow_mut())?;
        //                    self.inputs.push(new_source);
        //                }
        //                _ => {}
        //            };
        //                Err(err) => {
        //                    error!("Failed to load input {} : {}", next_input, err);
        //                }
        //                _ => {
        //                }
        //            }
        //        };
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

use thiserror::Error;
use rusqlite;
// use std::io;

// pub type Result<T> = std::result::Result<T, ZqError>;

#[derive(Error, Debug)]
pub enum ZqError {
    #[error("db error")]
    Db (#[from] rusqlite::Error)
    // #[fail(display = "{}", message)]
    // GeneralError { message: String },

    // #[fail(display = "{}", message)]
    // ParseError {
    //     message: String
    // },

    // #[fail(display = "{}", message)]
    // QueryError { message: String },

    // #[fail(display = "{}", message)]
    // IoError {
    //     message: String,
    //     backtrace: Backtrace,
    //     #[cause]
    //     cause: io::Error,
    // },

    // #[fail(display = "{}", message)]
    // SqLiteError {
    //     message: String,
    //     backtrace: Backtrace,
    //     #[cause]
    //     cause: rusqlite::Error,
    // },
}

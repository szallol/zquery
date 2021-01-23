use thiserror::Error;
use std::io;

pub type Result<T> = std::result::Result<T, ZqError>;

#[derive(Debug, Fail)]
pub enum ZqError {
    #[fail(display = "{}", message)]
    GeneralError { message: String },

    #[fail(display = "{}", message)]
    ParseError {
        message: String
    },

    #[fail(display = "{}", message)]
    QueryError { message: String },

    #[fail(display = "{}", message)]
    IoError {
        message: String,
        backtrace: Backtrace,
        #[cause]
        cause: io::Error,
    },

    #[fail(display = "{}", message)]
    SqLiteError {
        message: String,
        backtrace: Backtrace,
        #[cause]
        cause: rusqlite::Error,
    },
}

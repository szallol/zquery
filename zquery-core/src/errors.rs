use thiserror::Error;
use rusqlite;
use url;

#[derive(Error, Debug)]
pub enum ZqError {
    #[error("db error")]
    Db (#[source] rusqlite::Error),

    // #[fail(display = "{}", message)]
    // GeneralError { message: String },

     #[error("Parse error")]
    Parse(#[source] url::ParseError),

    // #[fail(display = "{}", message)]
    // QueryError { message: String },

    #[error("Io Error")]
    IoError( #[source] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ZqError>;
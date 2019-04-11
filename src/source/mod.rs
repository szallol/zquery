
pub mod sqlite;
pub use sqlite::ZqSqlite;

pub mod manager;

#[derive(PartialEq, Eq, Debug)]
pub enum ZqSourceError {
    InvalidUri
}

pub trait ZqSource {
}

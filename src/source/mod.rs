
pub mod sqlite;
pub use sqlite::ZqSqlite;

#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    InvalidUri
}

pub trait ZqSource {
    fn new() -> Result<ZqSource, Error > {
        Ok(Self{})
    }
}

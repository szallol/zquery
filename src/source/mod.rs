
pub mod sqlite;
pub use sqlite::ZqSqlite;

pub mod xml;
pub use xml::ZqXml;
use rusqlite::Error;

pub mod manager;

pub trait ZqCore {

}
pub trait ZqSource {
    fn import(&self);
}

use rusqlite::Error;

pub use crate::sqlite::ZqSqlite;
pub use crate::xml::ZqXml;

pub trait ZqCore {

}
pub trait ZqSource {
    fn import(&self);
}

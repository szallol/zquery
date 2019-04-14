use rusqlite::Error;

use crate::sqlite::ZqSqlite;
use crate::xml::ZqXml;
use crate::manager::ZqCore;


pub trait ZqSource {
    fn import(&self, core : &mut ZqCore);
}

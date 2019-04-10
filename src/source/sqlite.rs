use crate::source::ZqSource;
use crate::misc::ZqErr;


pub struct ZqSqlite {

}

impl  ZqSqlite {
    pub fn new () -> Result<ZqSqlite, ZqErr> {
        Ok(ZqSqlite{})
    }
}

impl ZqSource for ZqSqlite {

}
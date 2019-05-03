//use rusqlite::Error;

use crate::manager::ZqCore;

pub use crate::errors::*;

pub trait ZqSource {
    fn import(&self, core : &mut ZqCore) -> Result<()>;
}

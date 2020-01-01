use crate::manager::ZqCore;
use crate::errors::*;

pub mod xml;
pub mod sqlite;

pub trait ZqSource {
    fn import(&self, core : &mut dyn ZqCore) -> Result<()>;
}

use crate::errors::*;
use crate::zquery::ZqCore;

pub mod sqlite;
pub mod xml;

pub trait ZqSource {
    fn import(&self, core: &dyn ZqCore) -> Result<()>;
}

use crate::errors::*;
use crate::manager::ZqCore;

pub mod sqlite;
pub mod xml;

pub trait ZqSource {
    fn import(&self, core: &mut dyn ZqCore) -> Result<()>;
}
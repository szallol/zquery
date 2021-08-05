use crate::errors::*;
use crate::Zq;

pub mod sqlite;
pub mod xml;

pub trait ZqSource {
    fn import(&self, core: &Zq) -> Result<()>;
}

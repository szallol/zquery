use crate::manager::ZqCore;
use crate::errors::*;

pub trait ZqSource {
    fn import(&self, core : &mut ZqCore) -> Result<()>;
}

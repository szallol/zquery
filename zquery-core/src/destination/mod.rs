use crate::manager::ZqCore;
use crate::errors::*;

pub trait ZqDestination {
    fn export(&self, core : &mut ZqCore) -> Result<()>;
}


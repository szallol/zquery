use crate::ZqDone;
use crate::errors::*;

pub trait ZqDestination {
    fn export(&self, core : &mut ZqDone) -> Result<()>;
}

pub trait DestinationPool {
    fn add_destination<T : ZqDestination>(destination : T);
}

pub struct  ZqDestinationPool {}

impl DestinationPool for ZqDestinationPool {
    fn add_destination<T : ZqDestination>(_destination : T) {
        todo!()
    }
}

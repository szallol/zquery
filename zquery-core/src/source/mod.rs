use crate::errors::*;
use crate::Zq;

pub mod sqlite;
pub mod xml;

pub trait ZqSource {
    fn import(&self, core: &Zq) -> Result<()>;
}

pub trait SourcePool {
    fn add_source<T : ZqSource>(source : T);
}

pub struct  ZqSourcePool {}

    
impl SourcePool for ZqSourcePool {
    fn add_source<T : ZqSource>(_source : T) {
        todo!()
    }
}

impl  ZqSourcePool {
   pub fn new() -> Self {
    ZqSourcePool{}
   }

}


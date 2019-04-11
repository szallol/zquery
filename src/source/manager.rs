use url::{Url, ParseError};

use crate::source::ZqSqlite;
use crate::source::ZqSource;

pub struct Manager<'a> {
    pub inputs : Vec<&'a ZqSource>,
}

impl<'a> Manager<'a> {
    pub fn new () -> Manager<'a> {
        let inputs = Vec::new();
        Manager{inputs}
    }

    pub fn new_source(&'a mut self, source : String ) -> &'a mut Manager {
//        let input_url = Url::parse(source);
//        println! ("got url: {:?}", input_url);
        self
    }
}
use url::{Url, ParseError};

//use crate::source::ZqSqlite;
use crate::source::{ZqSource, ZqSqlite};

pub struct Manager {
    pub inputs : Vec<Box<dyn ZqSource>>,
}

//struct ManagerBuilder {
//    inputs : Vec<Box<dyn ZqSource>>,
//}

impl Manager {
    pub fn new () -> Manager {
        let inputs = Vec::new();
        Manager{inputs}
    }

    pub fn add_source(&mut self, source : &str ) -> Result<& Manager, ParseError> {
        let input_url = Url::parse(source)?;
        match input_url.scheme() {
           "xml" => {
               self.inputs.push(Box::new(ZqSqlite::new(input_url)))
           },
            _ => {}
        };
        Ok(self)
    }
}


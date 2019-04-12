use url::{Url, ParseError};

//use crate::source::ZqSqlite;
use crate::source::{ZqSource, ZqSqlite, ZqXml};

pub struct Manager{
    pub inputs : Vec<Box<dyn ZqSource>>,
}

impl Manager {
    pub fn new () -> Manager {
        let inputs = Vec::new();
        Manager{inputs}
    }

    pub fn add_source(&mut self, source : &str ) -> Result<&Manager, ParseError> {
        let input_url = Url::parse(source)?;
        match input_url.scheme() {
           "sqlite" => {
               let new_source  = Box::new(ZqSqlite::new(input_url));
               new_source.import();
               self.inputs.push(new_source);
           },
            "xml" => {
                let new_source  = Box::new(ZqXml::new(input_url));
                new_source.import();
                self.inputs.push(new_source);
            },
            _ => {}
        };
        Ok(self)
    }
}


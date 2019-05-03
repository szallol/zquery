extern crate env_logger;
extern crate log;

use log::*;
use url::Url;

pub use crate::errors::*;
use crate::source::ZqSource;
use crate::manager::ZqCore;

pub struct ZqXml {
    url: Url,
}

impl ZqXml {
    pub fn new(url: Url) -> Self {
        info!("new XML input: {}", url.to_string());
        ZqXml { url }
    }
}

impl ZqSource for ZqXml {
    fn import(&self, _core: &mut ZqCore) -> Result<(), Error> {
        info!("xml imported from: {}", self.url.to_string());
        Ok(())
    }
}

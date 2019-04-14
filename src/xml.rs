extern crate log;
extern crate env_logger;

use log::*;

use url::{Url};

use crate::source::ZqSource;

pub struct ZqXml {
    url : Url
}

impl ZqXml {
    pub fn new(url : Url) -> Self {
        info!("new XML input: {}", url.to_string());
        ZqXml{url}
    }
}

impl ZqSource for ZqXml {
    fn import (&self) {
        info!("xml imported from: {}", self.url.to_string());
    }
}

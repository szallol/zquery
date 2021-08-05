// use serde::{Serialize, Deserialize};
use url::{Url};
use crate::errors::{Result, ZqError};
use std::fmt;

#[derive(serde::Serialize)]
pub struct Config {
    inputs : Vec<InputConfig>,
    output : Option<OutputConfig>,
}

impl Config {
    pub fn new(inputs: Vec<InputConfig>, output: Option<OutputConfig>) -> Self { Self { inputs, output } }

    pub fn output(&self) -> &Option<OutputConfig> {&self.output}
}

#[derive(serde::Serialize)]
pub struct InputConfig {
    url : Url,

}

impl InputConfig {
    pub fn new(url: Url) -> Self { Self { url } }

    pub fn from_urls(urls : Vec<Url>) -> Result<Vec<InputConfig>> {
        Ok (
            urls
            .into_iter()
            .map(|url_ok| InputConfig::new(url_ok))
            .collect()
           )
    }

    pub fn from_strs(urls : Vec<&str>) -> Result<Vec<InputConfig>> {
        Ok( 
            urls
            .into_iter()
            .map(|url_str| Url::parse(url_str).unwrap())
            .map(|url_ok| InputConfig::new(url_ok))
            .collect()
          )
    }
}

#[derive(serde::Serialize)]
pub struct OutputConfig {
    url : Url,
}

impl OutputConfig {
    pub fn new(url: Url) -> Self { Self { url } }

    pub fn from_url(url : Url) -> Result<OutputConfig> {
        Ok(OutputConfig::new(url))
    }

    pub fn from_str(str_url : &str) -> Result<OutputConfig> {
        Ok(OutputConfig::new(Url::parse(str_url).map_err(ZqError::Parse)?))
    }
}

impl fmt::Display for OutputConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.url.as_str())
    }
}


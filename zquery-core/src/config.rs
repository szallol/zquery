// use serde::{Serialize, Deserialize};
use url::{Url};

#[derive(serde::Serialize)]
pub struct Config {
    input : Vec<InputConfig>,
}

impl Config {
    pub fn new(input: Vec<InputConfig>) -> Self { Self { input } }
}
#[derive(serde::Serialize)]
pub struct InputConfig {
    url : Url,
}

impl InputConfig {
    pub fn new(url: Url) -> Self { Self { url } }
}
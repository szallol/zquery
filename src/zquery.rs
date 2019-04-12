extern crate clap;

use clap::{ArgMatches};
use crate::source::{manager::Manager, ZqSource};

pub use misc::ZqErr;

pub use source::sqlite;

mod misc;
mod source;

pub struct ZQuery<'a>{
    pub args : ArgMatches<'a>,

    pub input_mgr : Manager,
}

impl<'a> ZQuery<'a> {
    pub fn new(args : ArgMatches) -> Result <ZQuery, misc::ZqErr> {
        let input_mgr = Manager::new();
        Ok (ZQuery {args, input_mgr})
    }

    pub fn run(mut self)
    {
        let input_values = self.args.values_of("input").unwrap();
        for input in input_values {
            self.input_mgr.add_source(input).unwrap();
        };
    }
}
extern crate clap;
use clap::{ArgMatches};

pub use crate::manager::Manager;
pub use crate::errors::*;

pub struct ZQuery<'a>{
    pub args : ArgMatches<'a>,

    pub input_mgr : Manager,
}

impl<'a> ZQuery<'a> {
    pub fn new(args : ArgMatches) -> ZQuery {
        let input_mgr = Manager::new();
        ZQuery {args, input_mgr}
    }

    pub fn run(mut self)
    {
        let input_values = self.args.values_of("input").unwrap();
        for input in input_values {
            self.input_mgr.add_source(input).unwrap();
        };
    }
}
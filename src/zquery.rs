extern crate clap;

use clap::{ArgMatches};

pub use misc::ZqErr;
pub use source::sqlite;

mod misc;
mod source;

pub struct ZQuery<'a> {
    pub args : ArgMatches<'a>,
}

impl<'a> ZQuery<'a> {
    pub fn new(matches : ArgMatches<'a>) -> Result <ZQuery<'a>, misc::ZqErr> {
        Ok (ZQuery {args : matches})
    }

    pub fn run(self)
    {
        let mut values = self.args.values_of("input").unwrap();
        println!("input values: {:?}", values);

    }
}
extern crate clap;

use clap::{ArgMatches};
use crate::source::{ZqSource, ZqSqlite};

pub use misc::ZqErr;

pub use source::sqlite;

mod misc;
mod source;

pub struct ZQuery<'a> {
    pub args : ArgMatches<'a>,

    pub inputs : Vec<&'a ZqSqlite >,
}
impl<'a> ZQuery<'a> {
    pub fn new(args : ArgMatches<'a>) -> Result <ZQuery<'a>, misc::ZqErr> {
        let mut inputs = Vec::new();
        Ok (ZQuery {args, inputs})
    }

    pub fn run(self)
    {
        let mut values = self.args.values_of("input").unwrap();
        println!("input values: {:?}", values);

    }
}
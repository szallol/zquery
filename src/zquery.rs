extern crate clap;
extern crate log;

use clap::{ArgMatches};

pub use misc::ZqErr;
pub use source::sqlite;
use crate::source::ZqSource;
use crate::source::ZqSqlite;

mod misc;
mod source;

pub struct ZQuery<'a> {
    pub args : ArgMatches<'a>,
//    pub inputs : Vec<&'a ZqSource>,
}

impl<'a, I : ZqSource> ZQuery<'a> {
    pub fn new(matches : ArgMatches<'a>) -> Result <ZQuery<'a>, misc::ZqErr> {
        let mut sources : Vec<dyn ZqSource> = vec![];
//        Ok (ZQuery {args : matches, sources})
        Ok (ZQuery {args : matches})
    }

    pub fn run(self)
    {
        let values = self.args.values_of("input").unwrap();
        println!("input values: {:?}", values);

    }
}
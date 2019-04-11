extern crate clap;

use clap::{ArgMatches};
use crate::source::{ZqSource, ZqSqlite, manager::Manager};

pub use misc::ZqErr;

pub use source::sqlite;

mod misc;
mod source;

pub struct ZQuery<'a> {
    pub args : ArgMatches<'a>,

    pub input_mgr : Manager<'a>,
//    pub inputs : Vec<&'a ZqSqlite >,
}
impl<'a> ZQuery<'a> {
    pub fn new(args : ArgMatches<'a>) -> Result <ZQuery<'a>, misc::ZqErr> {
        let mut input_mgr = Manager::new();
        input_mgr.new_source("sdfsdf".to_string());
        Ok (ZQuery {args, input_mgr})
    }

    pub fn run(self)
    {
        let kk : String = String::new();
        let mut values = self.args.values_of("input").unwrap();
//        values.for_each(|input| {
//            let input_url = input.to_owned();
//            self.input_mgr.new_source("sdfsdf");
//        });

    }
}
use log::*;
use clap::ArgMatches;

pub use crate::errors::*;
pub use crate::manager::Manager;

pub struct ZQuery<'a> {
    pub args: ArgMatches<'a>,

    pub input_mgr: Manager,
}

impl<'a> ZQuery<'a> {
    pub fn new(args: ArgMatches) -> Result<ZQuery> {
        let input_mgr = Manager::new()?;
        Ok(ZQuery { args, input_mgr })
    }

    pub fn run(mut self) -> Result<()> {
        let input_values = self.args.values_of("input").unwrap();
        for input in input_values {
            match self.input_mgr.add_source(input) {
                Err(err) => {
                    error!("Failed to load input {} : {}", input, err);
                }
                _ => {
                }
            }

        }

        Ok(())
    }
}

pub use misc::ZqErr;

mod misc;

pub struct ZQuery {

}

impl ZQuery {
    pub fn new() -> Result <ZQuery, misc::ZqErr> {
        Ok (ZQuery {})
    }
}
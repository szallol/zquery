use crate::zquery::column::ZqColumn;
use crate::ZqError;

pub struct ZqTable<'a> {
    name: &'a str,
    columns: Vec<ZqColumn<'a>>,
}

impl<'a> ZqTable<'a> {
    pub fn new(name: &'a str, columns: Vec<ZqColumn<'a>>) -> Self {
        ZqTable { name, columns }
    }

    pub fn name(&self) -> Result<&'a str, ZqError> {
        Ok(self.name)
    }

    pub fn columns(&self) -> Result<&Vec<ZqColumn>, ZqError> {
        Ok(&self.columns)
    }
}

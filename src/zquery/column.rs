use crate::ZqError;

pub struct ZqColumn<'a> {
    name: &'a str,
    sql_type: &'a str,
}

impl<'a> ZqColumn<'a> {
    pub fn new(name: &'a str, sqltype: &'a str) -> Self {
        ZqColumn {
            name,
            sql_type: sqltype,
        }
    }

    #[allow(dead_code)]
    pub fn name(&self) -> Result<&'a str, ZqError > {
        Ok(self.name)
    }

    #[allow(dead_code)]
    pub fn sql_type(&self) -> Result<&'a str, ZqError> {
        Ok(self.sql_type)
    }
}

extern crate rusqlite;

use rusqlite::types::ToSql;
use rusqlite::{Connection, Result, NO_PARAMS};


fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute("SELECT 2+2", NO_PARAMS);
    println!("Hello, world!");

    Ok(())
}

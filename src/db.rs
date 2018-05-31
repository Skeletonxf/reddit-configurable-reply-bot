extern crate rusqlite;

use self::rusqlite::Connection;

use LibResult;

/*
 * Database module to check and track replies to comments.
 */

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn replied(&self, id: &str) -> LibResult<bool> {
        let found: i32 = try!(self.connection.query_row(
            "SELECT EXISTS (
                SELECT 1 FROM replies WHERE id = ?1 LIMIT 1
            )", &[&id], |r| r.get(0))
        );
        Ok(found == 1)
    }

    pub fn reply(&self, id: &str) -> LibResult<()> {
        self.connection.execute("INSERT INTO replies (id) VALUES (?1)", &[&id])?;
        Ok(())
    }
}

pub fn from_connection(path: &str) -> LibResult<Database> {
    let connection = Connection::open(path)?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS replies (id TEXT PRIMARY KEY)", &[])?;
    Ok(Database {
        connection
    })
}

extern crate rawr;
extern crate rusqlite;

use rawr::traits::Content;

use self::rusqlite::Connection;

/*
 * Database module, to check, and track replies to comments
 */

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn replied(&self, content: &Content) -> Result<bool, rusqlite::Error> {
        let id = content.name();
        let found: i32 = try!(self.connection.query_row(
            "SELECT EXISTS (
                SELECT 1 FROM replies WHERE id = ?1 LIMIT 1
            )", &[&id], |r| r.get(0))
        );
        Ok(found == 1)
    }

    pub fn reply(&self, content: &Content) -> Result<(), rusqlite::Error> {
        let id = content.name();
        self.connection.execute("INSERT INTO replies (id) VALUES (?1)", &[&id])?;
        Ok(())
    }
}

pub fn from_connection(path: &str) -> Result<Database, rusqlite::Error> {
    let connection = Connection::open(path)?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS replies (id TEXT PRIMARY KEY)", &[])?;
    Ok(Database {
        connection
    })
}

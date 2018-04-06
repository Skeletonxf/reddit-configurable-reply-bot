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
    pub fn replied(&self, content: &Content) -> bool {
        false
    }

    pub fn reply(&self, content: &Content) {
        let id = content.name();
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

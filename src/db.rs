use rusqlite::{Connection, Result};
use std::sync::{Once};

pub const ONCE_INIT: Once = Once::new();

struct RemoteFileSystem {
    id: i32,
    ip: String,
    mount_point: String,
    is_connected: i32
}


fn establish_connection() -> Result<Connection> {
    Connection::open("example.sqlite")
}

// check if the db exists, if not create it.
fn init_db(){
    let db_file = "../db/sshfs.sqlite";

    if !db_file_exists(db_file){
        create_database(db_file)?;
    }
}

fn main() {
    ONCE_INIT.call_once(|| {
        init_db();
    });
}

fn create_record(){}

fn get_record(){}

fn update_record(){}

fn delete_record(){}

fn clear_all_records(){}

fn db_file_exists(file_path: &str) -> bool {
    std::path::Path::new(file_path).exists()
}

fn create_database(file_path: &str) -> Result<()> {
    // Open a connection to the SQLite database or create a new one
    let conn = Connection::open(file_path)?;

    // Create a table with a default schema
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sshfs (
                  id INTEGER PRIMARY KEY,
                  ip_addr TEXT NOT NULL,
                  is_connected INTEGER NOT NULL CHECK (is_connected IN (0, 1)
                  mount_point TEXT
                  )",

        [],
    )?;

    Ok(())
}
use std::fs;
use std::fmt;
use log::{info,error};
use rusqlite::{params, Connection, Result};

pub fn fetch_database_connection() -> Connection {

    let home_path = home::home_dir().unwrap();
    let _ = fs::create_dir_all(format!("{}/.config/lryp/", home_path.display().to_string()));

    let database_exists = fs::metadata(format!("{}/.config/lryp/lryp.db", home_path.display().to_string())).is_ok();
    
    // Open a connection to a new SQLite database
    let conn = Connection::open(format!("{}/.config/lryp/lryp.db", home_path.display().to_string())).unwrap();

    // Creates a new table if file doesn't exits
    if !database_exists {
        info!("Initializing configurations...");
        match conn.execute(
            "CREATE TABLE channels (
                id          INTEGER PRIMARY KEY,
                name        TEXT NOT NULL,
                channel_id  TEXT,
                url         TEXT NOT NULL
                )",
                [],
            ) {
                Ok(_) => {},
                Err(error) => error!("Error: {error:?}"),
            };
    }
 
    return conn;
}

pub fn fetch_channel_id(conn: &Connection, channel_url: &str) -> Option<Channel> {

    // Prepare the SQL query
    let mut stmt = conn.prepare("SELECT id, name, url, channel_id FROM channels where url = ?1 limit 1").ok()?;

    // Query the database and map the result to the User struct
    let channel_result = stmt.query_row([channel_url], |row| {
        Ok(Channel {
            id: row.get(0)?,
            name: row.get(1)?,
            url: row.get(2)?,
            channel_id: row.get(3)?
        })
    });

    // Handle the result
    match channel_result {
        Ok(channel) => Some(channel),
        Err(_) => None,
    }
}

pub fn update_channel_id(conn: &Connection, id: i32, channel_id: &str) -> Result<()> {
    conn.execute("UPDATE channels SET channel_id = ?1 WHERE id = ?2", params![channel_id, id],)?;
    Ok(())
} 

#[derive(Clone, Debug)]
pub struct Channel {
    pub id: i32,
    pub name: String,
    pub channel_id: Option<String>,
    url: String,
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
use std::fs;
use log::info;
use rusqlite::{params, Connection, Result};

pub fn connect() -> Result<()> {

    let home_path = home::home_dir().unwrap();
    let _ = fs::create_dir_all(format!("{}/.config/lryp/", home_path.display().to_string()));

    let database_exists = fs::metadata(format!("{}/.config/lryp/lryp.db", home_path.display().to_string())).is_ok();
    
    // Open a connection to a new SQLite database
    let conn = Connection::open(format!("{}/.config/lryp/lryp.db", home_path.display().to_string()))?;

    // Creates a new table if file doesn't exits
    if !database_exists {
        info!("Initializing configurations...");
        let result = match conn.execute(
            "CREATE TABLE channels (
                id          INTEGER PRIMARY KEY,
                name        TEXT NOT NULL,
                channel_id  TEXT,
                url         TEXT NOT NULL
                )",
                [],
            ) {
                Ok(stuff) => println!("been here"),
                Err(error) => println!("Error: {error:?}"),
            };
    }
        
        
    // Insert some data
    conn.execute(
        "INSERT INTO channels (name, url) VALUES (?1, ?2)",
        params!["Super GT", "https://www.youtube.com/@Super_GT"],
    )?;
    conn.execute(
        "INSERT INTO channels (name, url) VALUES (?1, ?2)",
        params!["TitusTechTalk", "https://www.youtube.com/@TitusTechTalk"],
    )?;

    // Query the data
    let mut stmt = conn.prepare("SELECT id, name, url FROM channels")?;
    let channel_iter = stmt.query_map([], |row| {
        Ok(Channel {
            id: row.get(0)?,
            name: row.get(1)?,
            url: row.get(2)?,
            channel_id: None
        })
    })?;

    // Print the results
    for channel in channel_iter {
        println!("Found channel {:?}", channel?);
    }

    Ok(())
}

#[derive(Debug)]
struct Channel {
    id: i32,
    name: String,
    channel_id: Option<String>,
    url: String,
}
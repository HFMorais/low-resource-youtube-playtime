use std::fs;
use std::path::Path;
use rusqlite::{params, Connection, Result};

pub fn connect() -> Result<()> {

    let home_path = home::home_dir().unwrap();
    let _ = fs::create_dir_all(format!("{}/.config/lryp/", home_path.display().to_string()));


    let database_exists = fs::metadata(format!("{}/.config/lryp/lryp.db", home_path.display().to_string())).is_ok();
    

    // Open a connection to a new SQLite database
    let conn = Connection::open(format!("{}/.config/lryp/lryp.db", home_path.display().to_string()))?;

    // Creates a new table if file doesn't exits
    if !database_exists {
        println!("database exists");
        let result = match conn.execute(
            "CREATE TABLE person (
                id      INTEGER PRIMARY KEY,
                name    TEXT NOT NULL,
                data    BLOB
                )",
                [],
            ) {
                Ok(stuff) => println!("been here"),
                Err(error) => println!("Error: {error:?}"),
            };
    }
        
        
    // Insert some data
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        params!["Alice", "Some data about Alice"],
    )?;
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        params!["Bob", "Some data about Bob"],
    )?;

    // Query the data
    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    // Print the results
    for person in person_iter {
        println!("Found person {:?}", person?);
    }

    Ok(())
}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: String,
}
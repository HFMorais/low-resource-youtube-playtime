use std::fs;
use log::{info,error};
use rusqlite::{params, Connection, Result};
use crate::data_structures::Channel;

use super::data_structures::VideoEntry;

pub fn fetch_database_connection() -> Connection {

    let home_path = home::home_dir().unwrap();
    let _ = fs::create_dir_all(format!("{}/.config/lryp/", home_path.display().to_string()));

    let database_exists = fs::metadata(format!("{}/.config/lryp/lryp.db", home_path.display().to_string())).is_ok();
    
    // Open a connection to a new SQLite database
    let conn = Connection::open(format!("{}/.config/lryp/lryp.db", home_path.display().to_string())).unwrap();

    // Creates a new table if file doesn't exits
    if !database_exists {
        info!("Initializing configurations...");
        // Create table for channels
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

        // Create table for video entries
        match conn.execute(
            "CREATE TABLE videos (
                id          INTEGER PRIMARY KEY,
                name        TEXT NOT NULL,
                channel_id  INTEGER NOT NULL,
                video_id    TEXT NOT NULL,
                time_added  INTEGER,
                seen        INTEGER,
                FOREIGN KEY(channel_id) REFERENCES channels(id)
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

//pub fn save_channel_info(conn: &Connection, name: &str, url: &str, channel_id: &str) -> Result<Channel> {
pub fn save_channel_info(conn: &Connection, channel: &Channel) -> Result<i64> {
    conn.execute(
        "INSERT INTO channels (name, channel_id, url) VALUES (?1, ?2, ?3)",
        params![channel.name, channel.channel_id, channel.url],
    )?;

    Ok(conn.last_insert_rowid())
}

/*
 * Add video to database, if it already exists there then do nothing
 * If the return value is -1, the video already was in database.
 */ 
pub fn add_video_to_database(conn: &Connection, video_entry: &VideoEntry) -> Result<i64> {
    // Check if the video already exists in the database
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM videos WHERE video_id = ?1",
        params![video_entry.video_id],
        |row| row.get(0),
    )?;

    // If the video already exists, return early
    if count > 0 {
        return Ok(-1);
    }

    conn.execute(
        "INSERT INTO videos (name, channel_id, video_id, time_added, seen) VALUES (?1, ?2, ?3, ?4, ?5)", 
        params![
            video_entry.title,
            video_entry.channel_id,
            video_entry.video_id,
            video_entry.added_date,
            video_entry.seen
        ]
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn fetch_videos_from_channel(conn: &Connection, channel_id: &i64) -> Result<Vec<VideoEntry>> {
    let mut videos = Vec::new();
    let mut stmt = conn.prepare(
        "SELECT id, name, channel_id, video_id, time_added, seen FROM videos WHERE channel_id = ?1 and seen = ?0 order by id desc"
    )?;
    
    let rows = stmt.query_map(params![channel_id, 0], |row| {
        Ok(VideoEntry {
            id: row.get(0)?,
            title: row.get(1)?,
            channel_id: row.get(2)?,
            video_id: row.get(3)?,
            added_date: row.get(4)?,
            seen: row.get(5)?,
        })
    })?;

    for video in rows {
        videos.push(video?);
    }

    Ok(videos)
}
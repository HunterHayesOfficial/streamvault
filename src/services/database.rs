use rusqlite::{Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Streamer {
    pub id: i64,
    pub name: String,
    pub channel_id: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn init(db_path: &str) -> Result<Self, Box<dyn Error>> {
        let path = Path::new(db_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS streamers (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                channel_id TEXT NOT NULL UNIQUE
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn get_streamers(&self) -> SqliteResult<Vec<Streamer>> {
        let mut stmt = self.conn.prepare("SELECT id, name, channel_id FROM streamers")?;
        let streamer_iter = stmt.query_map([], |row| {
            Ok(Streamer {
                id: row.get(0)?,
                name: row.get(1)?,
                channel_id: row.get(2)?,
            })
        })?;

        let mut streamers = Vec::new();
        for streamer in streamer_iter {
            streamers.push(streamer?);
        }
        Ok(streamers)
    }

    pub fn add_streamer(&self, name: &str, channel_id: &str) -> SqliteResult<()> {
        self.conn.execute(
            "INSERT INTO streamers (name, channel_id) VALUES (?1, ?2)",
            &[name, channel_id],
        )?;
        Ok(())
    }
}
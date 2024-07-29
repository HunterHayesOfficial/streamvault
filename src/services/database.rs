use rusqlite::{Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::error::Error;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Streamer {
    pub id: i64,
    pub name: String,
    pub channel_id: String,
}

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
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
        Ok(Self { conn: Arc::new(Mutex::new(conn)) })
    }

    pub fn get_streamers(&self) -> SqliteResult<Vec<Streamer>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, channel_id FROM streamers")?;
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
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO streamers (name, channel_id) VALUES (?1, ?2)",
            &[name, channel_id],
        )?;
        Ok(())
    }

    pub fn remove_streamer(&self, channel_id: &str) -> SqliteResult<bool> {
        let conn = self.conn.lock().unwrap();
        let rows_affected = conn.execute(
            "DELETE FROM streamers WHERE channel_id = ?1",
            &[channel_id],
        )?;
        Ok(rows_affected > 0)
    }
}
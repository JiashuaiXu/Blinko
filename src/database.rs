// Simple database for statistics

use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub fn init() -> anyhow::Result<()> {
    let db_path = db_path();
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS reminders (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp INTEGER NOT NULL,
            type TEXT NOT NULL,
            action TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS statistics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL UNIQUE,
            reminder_count INTEGER DEFAULT 0,
            rest_count INTEGER DEFAULT 0,
            blink_alerts INTEGER DEFAULT 0,
            posture_alerts INTEGER DEFAULT 0
        )",
        [],
    )?;

    Ok(())
}

pub fn record_reminder(reminder_type: &str, action: Option<&str>) -> anyhow::Result<()> {
    let conn = Connection::open(db_path())?;
    let timestamp = chrono::Utc::now().timestamp();
    
    conn.execute(
        "INSERT INTO reminders (timestamp, type, action) VALUES (?1, ?2, ?3)",
        [&timestamp.to_string(), reminder_type, action.unwrap_or("")],
    )?;

    // Update daily statistics
    let date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    conn.execute(
        "INSERT INTO statistics (date, reminder_count) VALUES (?1, 1)
         ON CONFLICT(date) DO UPDATE SET reminder_count = reminder_count + 1",
        [&date],
    )?;

    Ok(())
}

fn db_path() -> PathBuf {
    let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("blinko");
    path.push("blinko.db");
    path
}


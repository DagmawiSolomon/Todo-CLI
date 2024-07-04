use std::result;

use rusqlite::{ffi::Error, params, params_from_iter, Connection, Result, ToSql};
use crate::models::Operations;
use crate::models::{self, Category};

pub fn create_tables() -> Result<()> {
    let conn = Connection::open("todocli.db")?;

    // Enable foreign key constraints
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Status (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL UNIQUE,
            color TEXT NOT NULL DEFAULT '#000000'
        )",
        [],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO Status (title, color) VALUES
        ('Pending','#FFA500'), 
        ('Not Started','#808080'), 
        ('In Progress', '#0000FF'), 
        ('Completed', '#008000')",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Priority (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL UNIQUE,
            color TEXT NOT NULL DEFAULT '#000000' 
        )",
        [],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO Priority (title, color) VALUES
        ('High', '#FF0000'),
        ('Medium', '#FFFF00'),
        ('Low', '#00FF00')",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Category (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            color TEXT,
            emoji TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Tag (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            color TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Task (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            created_at TEXT NOT NULL,
            last_updated TEXT NOT NULL,
            priority TEXT NOT NULL,
            due_date TEXT NOT NULL,
            status_id INTEGER NOT NULL,
            category_id INTEGER NOT NULL,
            FOREIGN KEY(status_id) REFERENCES Status(id),
            FOREIGN KEY(category_id) REFERENCES Category(id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS TaskTag (
            task_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (task_id, tag_id),
            FOREIGN KEY (task_id) REFERENCES Task(id),
            FOREIGN KEY (tag_id) REFERENCES Tag(id)
        )",
        [],
    )?;

    Ok(())
}




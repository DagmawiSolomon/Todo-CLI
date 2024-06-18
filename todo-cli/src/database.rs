use rusqlite::{Connection, Result};
use crate::models;

pub fn create_tables() -> Result<()> {
    let conn = Connection::open("todocli.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Color (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             Red INTEGER NOT NULL,
             Green INTEGER NOT NULL,
             Blue INTEGER NOT NULL
         )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Status (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT DEFAULT 'Not Started'
         )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Category (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            color_id INTEGER NOT NULL,
            emoji TEXT NOT NULL,
            FOREIGN KEY(color_id) REFERENCES Color(id)
         )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Tag (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            color_id INTEGER NOT NULL,
            FOREIGN KEY(color_id) REFERENCES Color(id)
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


pub fn add_task(task:models::Task) -> Result<()>{
    let con = Connection::open("todocli.db")?;
    con.execute("INSERT INTO \"Color\" (Red, Green, Blue) VALUES (?1, ?2, ?3);", [255, 255, 255])?;
    Ok(())

}


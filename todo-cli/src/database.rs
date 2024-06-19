use core::task;

use rusqlite::{ffi::Error, Connection, Result};
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
            title TEXT NOT NULL UNIQUE
         )",
        [],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO Status (title) VALUES 
        ('Pending'), 
        ('Not Started'), 
        ('In Progress'), 
        ('Completed')",
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
    // let color_id = add_color(&con);
    add_status(&con, task);
    // println!("{}", color_id);
    Ok(())

}


pub fn add_color(con : &Connection) -> i64{
    con.execute("INSERT INTO Color (Red, Green, Blue) VALUES (?1, ?2, ?3);", [255, 255, 255])
    .expect("Troubel Creating COlor");

    con.last_insert_rowid()

    
}


pub fn add_status(con: &Connection, task: models::Task) -> Result<i64, rusqlite::Error> {
    let status: Result<usize, rusqlite::Error> = con.execute(
        "INSERT INTO Status (title) VALUES (?1)",
        &[&task.title],
    );

    match status {
        Ok(_) => Ok(con.last_insert_rowid()),
        Err(err) => Err(err),
    }
}


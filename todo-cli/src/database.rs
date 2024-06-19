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
    // let status_id = add_status(&con, task);
    let status_id = get_status(&con, &task.status);
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

pub fn get_status(con: &Connection, status: &models::Status) -> Result<i32> {
    let mut stmt = con.prepare("SELECT id FROM Status WHERE UPPER(title) == UPPER(?1)")?;
    let mut rows = stmt.query(&[&status.title])?;

    if let Some(row) = rows.next()? {
        let id: i32 = row.get(0)?;
        println!("{}",id);
        Ok(id)
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
        
    }
}



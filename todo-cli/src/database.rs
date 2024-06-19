use core::task;

use rusqlite::{ffi::Error, Connection, Result};
use crate::models::{self, Category};

pub fn create_tables() -> Result<()> {
    let conn = Connection::open("todocli.db")?;

    // Enable foreign key constraints
    conn.execute("PRAGMA foreign_keys = ON", [])?;

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



pub fn add_task(task:models::Task) -> Result<()>{
    let con = Connection::open("todocli.db")?;
    // let status_id = add_status(&con, task);
    // let status_id = get_status(&con, &task.status);
    let category = add_category(&con, &task.category);
    println!("{:?}", category);
    Ok(())

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

pub fn get_status(con: &Connection, status: &models::Status) -> Result<i64> {
    let mut stmt = con.prepare("SELECT id FROM Status WHERE UPPER(title) == UPPER(?1)")?;
    let mut rows = stmt.query(&[&status.title])?;

    if let Some(row) = rows.next()? {
        let id: i64 = row.get(0)?;
        println!("{}",id);
        Ok(id)
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
        
    }
}

pub fn add_category(con: &Connection, category: &models::Category) -> Result<i64, rusqlite::Error>{
    let category = con.execute(
        "INSERT INTO Category (title, emoji, color) VALUES (?1,?2,?3)",
        &[&category.title,&category.emoji, &category.color]
    );

    match category {
        Ok(_) => Ok(con.last_insert_rowid()),
        Err(err) => Err(err),
    }

}

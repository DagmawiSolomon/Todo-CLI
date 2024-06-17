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
    let con = Connection::open_in_memory()?;
    let task_status = add_status(&con, task.status);
    con.execute("INSERT INTO Task (title, description, created_at, last_updated, priority, due_date, status_id, category_id );"
    ,&[&task.title, &task.descripton, &task.created_at.to_string(), &task.last_updated.to_string(), &task.prority, &task.due_date, &task.status.title, &task.category.title])?;
    Ok(())
}

fn add_status(con: &Connection, status: models::Status)-> Result<()>{
    con.execute("INSERT INTO Status (title);", [status.title])?;

    con.prepare("SELECT * FROM Status where title(title)");
    Ok(())
}
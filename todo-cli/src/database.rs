use std::result;

use rusqlite::{ffi::Error, Connection, Result, params, ToSql};
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


pub fn add_task(task:models::Task) -> Result<()>{
    let con = Connection::open("todocli.db")?;
    // let status_id = add_status(&con, task);
    // let status_id = get_status(&con, &task.status);

    // let table = "Category";
    // let fields = vec!{"title","color"};
    // let values: &[&dyn ToSql] = params!{"Hello","#323031"};
    // let category = add(&con, "Category", fields, values);
    // let status = create(&con, "Status",vec!{"title","color"},params!(&task.status.title, &task.status.color));
    let stat = retrieve(&con, "SELECT id FROM Status WHERE UPPER(title) == Upper('New')");
    println!("{:?}",stat);
    Ok(())

}

fn create_placeholder(len: usize) -> String{
    (1..=len).map(|i| format!("?{}", i)).collect::<Vec<_>>().join(",")
}

pub fn create(con: &Connection, table: &str, fields: Vec<&str>, params: &[&dyn ToSql]) -> Result<i64, rusqlite::Error>{
    let fields_list = fields.join(",");
    let value_placeholders = create_placeholder(params.len());
    let sql = format!("INSERT INTO {} ({}) VALUES ({})", table, fields_list, value_placeholders);
    let result = con.execute(&sql, params);
    match result {
        Ok(_) => Ok(con.last_insert_rowid()),
        Err(err) => Err(err),
    }
}

pub fn retrieve(con: &Connection, sql: &str) -> Result<i64> {
    let mut stmt = con.prepare(sql)?;
    let mut rows = stmt.query([])?;
    if let Some(row) = rows.next()? {
        let id: i64 = row.get(0)?;
        println!("{}",id);
        Ok(id)
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
        
    }
}






// add a check for null categories
pub fn get_category(con: &Connection, category: &models::Category)  -> Result<i64>{
    let mut stmt = con.prepare("SELECT id FROM category WHERE UPPER(title) = UPPER(?1)")?;
    let mut rows = stmt.query(&[&category.title])?;

    if let Some(row) = rows.next()? {
        let id:i64 = row.get(0)?;
        println!("{}", id);
        Ok(id)
    }
    else{
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}


// TODO CREATE Functions Create, Retieve, Update and Delete instead of get_category
// Create a test for each function
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


pub fn add_task(task:models::Task) -> Result<()>{
    let con = Connection::open("todocli.db")?;
    // let status_id = add_status(&con, task);
    // let status_id = get_status(&con, &task.status);

    // let table = "Category";
    // let fields = vec!{"title","color"};
    // let values: &[&dyn ToSql] = params!{"Hello","#323031"};
    // let category = add(&con, "Category", fields, values);
    // let status = create(&con, "Status",vec!{"title","color"},params!(&task.status.title, &task.status.color));
    // let stat = retrieve(&con, "SELECT id FROM Status WHERE UPPER(title) == Upper('New')");
    
    // let fields = vec!{"title","description","created_at", "last_updated", "priority", "due_date", "status_id", "category_id"};
    // let status_id = create(&con,"Status",vec!{"title", "color"}, params!{task.status.title, task.status.color}).unwrap();
    // let category_id = create(&con,"Category",vec!{"title", "color"}, params!{task.category.title, task.category.color}).unwrap();
    // let values = params!{task.title, task.description, task.created_at, task.last_updated, task.prority, task.due_date, status_id, category_id};
    // let task = create(&con, "Task", fields, values);

    // let task = get_or_create(con, model);
    // println!("{:?}",task);
    let x = Category {
        title: String::from("Some Title"),
        color: String::from("Red"),
        emoji: String::from("😊"),
    };
    x.create();

    Ok(())

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


fn create_placeholder(len: usize) -> String{
    (1..=len).map(|i| format!("?{}", i)).collect::<Vec<_>>().join(",")
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


pub fn update(con:&Connection, sql: &str) -> Result<(), rusqlite::Error>{
    let result = con.execute(sql, []);

    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}


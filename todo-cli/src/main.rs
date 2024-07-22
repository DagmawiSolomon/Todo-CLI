#[macro_use]
extern crate diesel;

mod schema;
mod models;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;
use crate::models::{Task, Tag, TaskTag};
use chrono::Utc;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_task(conn: &SqliteConnection, task: Task) -> Task {
    use schema::tasks;

    diesel::insert_into(tasks::table)
        .values(&task)
        .execute(conn)
        .expect("Error saving new task");

    tasks::table.order(tasks::id.desc()).first(conn).unwrap()
}

pub fn read_tasks(conn: &SqliteConnection) -> Vec<Task> {
    use schema::tasks::dsl::*;

    tasks.load::<Task>(conn).expect("Error loading tasks")
}

pub fn update_task(conn: &SqliteConnection, task_id: i32, updated_task: Task) -> Task {
    use schema::tasks::dsl::*;

    diesel::update(tasks.find(task_id))
        .set(&updated_task)
        .execute(&conn)
        .expect("Error updating task");

    tasks.find(task_id).first(conn).unwrap()
}

pub fn delete_task(conn: &SqliteConnection, task_id: i32) {
    use schema::tasks::dsl::*;

    diesel::delete(tasks.find(task_id))
        .execute(conn)
        .expect("Error deleting task");
}

fn main() {
    let connection = establish_connection();

    let new_task = Task {
        id: 0, // Diesel will auto-increment this
        title: "New Task".to_string(),
        description: Some("Description of the new task".to_string()),
        created_at: Utc::now().naive_utc(),
        last_updated: Utc::now().naive_utc(),
        status_title: Some("Open".to_string()),
        status_color: Some("#FF0000".to_string()),
        priority: Some("High".to_string()),
        due_date: Some("2024-12-31".to_string()),
        category_title: Some("Work".to_string()),
        category_color: Some("#00FF00".to_string()),
        category_emoji: Some("💼".to_string()),
    };

    // Create
    let created_task = create_task(&connection, new_task);
    println!("Created Task: {:?}", created_task);

    // Read
    let tasks = read_tasks(&connection);
    println!("All Tasks: {:?}", tasks);

    // Update
    let updated_task = Task {
        title: "Updated Task".to_string(),
        ..created_task.clone()
    };
    let updated_task = update_task(&connection, created_task.id, updated_task);
    println!("Updated Task: {:?}", updated_task);

    // Delete
    delete_task(&connection, updated_task.id);
    println!("Task deleted");
}

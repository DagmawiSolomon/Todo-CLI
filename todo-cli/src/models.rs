use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};
use crate::schema::{tasks, tags, task_tags};
use chrono::NaiveDateTime;

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "tasks"]
pub struct Task {
    pub id: Option<i32>, // Change to Option<i32> for auto-increment handling
    pub title: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub last_updated: NaiveDateTime,
    pub status_title: Option<String>,
    pub status_color: Option<String>,
    pub priority: Option<String>,
    pub due_date: Option<String>,
    pub category_title: Option<String>,
    pub category_color: Option<String>,
    pub category_emoji: Option<String>,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "tags"]
pub struct Tag {
    pub id: Option<i32>, // Option<i32> if auto-incremented
    pub title: String,
    pub color: String,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "task_tags"]
pub struct TaskTag {
    pub task_id: i32,
    pub tag_id: i32,
}

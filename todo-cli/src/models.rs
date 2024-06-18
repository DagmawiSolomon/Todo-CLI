use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Color(pub i64,pub i64, pub i64);
#[derive(Debug)]
pub struct Status{
    pub title:String,
    // color: Color
}

// pending inprogress Done 
#[derive(Debug)]
pub struct Category{
    pub title:String,
    pub color: Color,
    pub emoji: String,
}
#[derive(Debug)]
pub struct Tag{
    pub title: String,
    pub color: Color,
}

impl Tag{
    pub fn new(tags: Vec<String>) -> Vec<Tag>{
        let mut list: Vec<Tag> = Vec::new();
        for tag in tags{
            list.push(Tag{
                title: tag,
                color: Color(0,0,0),
            })
        }
        list
    } 
}

#[derive(Debug)]
pub struct Task{
    pub title: String,
    pub descripton: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub status: Status,
    pub prority: String,
    pub due_date: String,
    pub category:Category,
    pub tags: Vec<Tag>,
}

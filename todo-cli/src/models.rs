
pub struct Color(i64,i64,i64);

pub struct Status{
    pub title:String,
    // color: Color
}

pub struct Category{
    pub title:String,
    pub descripton:String,
    pub color: Color,
    pub emoji: String,
}

pub struct Tag{
    pub title: String,
    pub color: Color,
}

struct Task{
    pub title: String,
    pub descripton: String,
    pub created_at: String,
    pub last_updated: String,
    pub status: Status,
    pub prority: String,
    pub due_date: String,
    pub category:Category,
    pub tags: Tag,
}

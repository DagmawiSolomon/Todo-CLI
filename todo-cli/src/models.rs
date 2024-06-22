#[derive(Debug)]
pub struct Status{
    pub title:String,
    pub color: String,
}

// pending inprogress Done 
#[derive(Debug)]
pub struct Category{
    pub title:String,
    pub color: String,
    pub emoji: String,
}
#[derive(Debug)]
pub struct Tag{
    pub title: String,
    pub color: String,
}

impl Tag{
    pub fn new(tags: Vec<String>) -> Vec<Tag>{
        let mut list: Vec<Tag> = Vec::new();
        for tag in tags{
            list.push(Tag{
                title: tag,
                color: "#323031".to_string(),
            })
        }
        list
    } 
}

#[derive(Debug)]
pub struct Task{
    pub title: String,
    pub description: String,
    pub created_at: String,
    pub last_updated: String,
    pub status: Status,
    pub prority: String,
    pub due_date: String,
    pub category:Category,
    pub tags: Vec<Tag>,
}

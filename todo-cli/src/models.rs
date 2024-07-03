use std::fmt::Debug;
use std::any::type_name;

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
    pub due_date: String, // tommorow // next week // blank - todays date // yyyy-mm-dd
    pub category:Category,
    pub tags: Vec<Tag>,
}


pub trait Operations{
    type Item;

    fn create(&self, item: &Self::Item)  where
    Self::Item: Debug,{
        println!("{:#?}", item);
    }
    fn get(){}
    fn all(){}
    fn sort(){}
    fn filter(){}
    fn update(){}
    fn delete(){}

}


impl <T> Operations for T {
    type Item = T;
}

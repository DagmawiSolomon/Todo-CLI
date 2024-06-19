use rusqlite::{Connection, Result};
use colored::*;
use chrono::Utc;
use structopt::StructOpt;
mod args;
mod models;
mod database;
fn main() {
    database::create_tables().expect("Error Creating Database Tables");
    let args = args::Args::from_args();
    match args.commands {
        args::Command::add {task,due,description,priority,category,emoji,tags} =>{
            let x = models::Task{
                title: task,
                descripton: match description{
                    None => " ".to_string(),
                    Some(x) => x,
                },
                status: models::Status{
                    title: "Pending".to_string(),
                },
                due_date: match due {
                    None => " ".to_string(),
                    Some(x) => x,
                },
                created_at: Utc::now(),
                last_updated: Utc::now(),
                prority: match priority {
                    None => " ".to_string(),
                    Some(x) => x,
                },
                category:models::Category{
                    title: match category {
                        None => " ".to_string(),
                        Some(x) => x,
                    },
                    color: models::Color(0,0,0),
                    emoji: match emoji{
                        None => " ".to_string(),
                        Some(x) => x,
                    },
                },
                tags: models::Tag::new(tags), 
            };
            let x = database::add_task(x);
            println!("{:?}", x);
          
        }
        _ => println!("Mismatch")
    }

   

}

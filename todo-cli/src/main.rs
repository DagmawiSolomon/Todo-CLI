use chrono::Utc;
use structopt::StructOpt;
mod args;
mod models;
mod database;
fn main() {
   
    let create_dbtables = database::create_tables();
    match create_dbtables {
    Err(err) => panic!("Error creating database: {}", err),
    _ => (),
    }

    let args = args::Args::from_args();
    match args.commands {
        args::Command::Add {task,due,description,priority,category,emoji,tags} =>{
            let x = models::Task{
                title: task,
                description: match description{
                    None => " ".to_string(),
                    Some(x) => x,
                },
                status: models::Status{
                    title: "Older".to_string(),
                    color: "#323031".to_string(),
                },
                due_date: match due {
                    None => " ".to_string(),
                    Some(x) => x,
                },
                created_at: Utc::now().to_string(),
                last_updated: Utc::now().to_string(),
                prority: match priority {
                    None => " ".to_string(),
                    Some(x) => x,
                },
                category:models::Category{
                    title: match category {
                        None => " ".to_string(),
                        Some(x) => x,
                    },
                    color: "#323031".to_string(),
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

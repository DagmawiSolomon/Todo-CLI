use colored::*;
mod models;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let task = models::Status{
        title:"hello".to_string(),
        
    };
   println!("{}", task.title);
}

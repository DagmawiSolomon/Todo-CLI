use colored::*;
mod models;
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
// struct addflags{
//     due: String,
//     category: String,
//     proprity: String,

// }
#[derive(Debug, StructOpt)]
struct Add{
    command: String,
    title: String,
    

}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let task = models::Status{
        title:"hello".to_string(),
        
    };
   println!("{}", task.title);
}

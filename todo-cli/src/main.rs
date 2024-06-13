use colored::*;
mod models;
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
struct addflags{
    due: String,
    category: String,
    proprity: String,

}
#[derive(Debug, StructOpt)]
struct Add{
    command: String,
    title: String,
    flags: addflags,

}
fn main() {
    let task = models::Status{
        title:"hello".to_string(),
        
    };
    let opts = Add::from_args();
   println!("{:?}", opts);
}

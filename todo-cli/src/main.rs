use colored::*;
use structopt::StructOpt;
mod args;

fn main() {
    let args = args::Args::from_args();
    println!("{:?}", args);
}

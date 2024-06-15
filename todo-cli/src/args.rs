use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Command {
    add {
        #[structopt(help = "Description of the task")]
        task: String,
        #[structopt(short, long, help = "Deadline of task")]
        due: Option<String>,
        #[structopt(short, long, help = "Level of priority: low, medium, high")]
        priority: Option<String>,
        #[structopt(short, long, help = "Category of task")]
        category: Option<String>,
        #[structopt(short, long, help = "Tags for task")]
        tags: Vec<String>,
    },
    list {
        #[structopt(short, long)]
        due: Option<String>,
        #[structopt(short, long)]
        category: Option<String>,
        #[structopt(short, long)]
        tags: Vec<String>,
        #[structopt(short, long)]
        status: Option<String>,
        #[structopt(short, long)]
        priority: Option<String>,
        // title - contains starts with ends with
    },
    sort{
        #[structopt(short, long)]
        due: Option<String>,
        #[structopt(short, long)]
        title: Option<String>,
        
    },
    status{
        id: Option<i64>,
        title: Option<String>,
        status: String,
    },
    edit {
        #[structopt(help = "ID of the task")]
        id: Option<i64>,
        #[structopt(help = "Description of the task")]
        task: Option<String>,
        #[structopt(short, long, help = "Deadline of task")]
        due: Option<String>,
        #[structopt(short, long, help = "Level of priority: low, medium, high")]
        priority: Option<String>,
        #[structopt(short, long, help = "Category of task")]
        category: Option<String>,
        #[structopt(short, long, help = "Tags for task")]
        tags: Vec<String>,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name="Fundamentals", author = "Dagmawi Solomon", about= "I am a simple CLI to teach you the fundamentals")]
pub struct Args {
    #[structopt(subcommand)]
    commands: Command,
}
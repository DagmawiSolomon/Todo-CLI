use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    Add {
        #[structopt(help = "Description of the task")]
        task: String,
        #[structopt(short="D", long, help = "Description of task")]
        description: Option<String>,
        #[structopt(short, long, help = "Deadline of task (dd-mm-yyyy) or tommorow")]
        due: Option<String>,
        #[structopt(short, long, help = "Level of priority: low, medium, high")]
        priority: Option<String>,
        #[structopt(short, long, help = "Category of task")]
        category: Option<String>,
        #[structopt(short, long, help = "Emoji for category")]
        emoji: Option<String>,
        #[structopt(short, long, help = "Tags for task")]
        tags: Vec<String>,
    },
    List {
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
    Sort{
        #[structopt(short, long)]
        due: Option<String>,
        #[structopt(short, long)]
        title: Option<String>,
        
    },
    Status{
        id: Option<i64>,
        title: Option<String>,
        status: String,
    },
    Edit {
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
#[structopt(name="Fundamentals", author = "Dagmawi Solomon", about= "A simple cli todo app")]
pub struct Args {
    #[structopt(subcommand)]
    pub commands: Command,
}

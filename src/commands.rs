use clap::Args;

#[derive(Debug, Args)]
pub struct AddTaskCommand {
    #[arg(short, long)]
    pub name: String,
    #[arg(short, long)]
    pub group: String,
}

#[derive(Debug, Args)]
pub struct ListTaskCommand {
    #[arg(short, long)]
    pub group: Option<String>,
}

#[derive(Debug, Args)]
pub struct AddGroupCommand {
    #[arg(short, long)]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct ListGroupCommand {}

#[derive(Debug, Args)]
pub struct AddTimeCommand {
    #[arg(short, long)]
    pub task: String,
}

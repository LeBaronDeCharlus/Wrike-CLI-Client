use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "wrs")]
#[command(author = "Le Baron de Charlus <f00b4rch@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "access Wripe through CLI", long_about = None)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Actions on tasks, default list your tasks
    Tasks(Tasks),
    // Folders(Folders),
}

#[derive(Debug, Parser)]
pub struct Tasks {
    /// Search tasks, matching words in title
    #[arg(long)]
    pub search: Option<String>,
    /// Filter searched tasks by status
    #[arg(long)]
    pub status: Option<String>,
    /// Filter by only looking for your tasks
    #[arg(short, long)]
    pub me: bool,
}

#[derive(Args, Debug)]
pub struct Folders {
    pub folders_arg: Option<String>,
}
//#[derive(Args, Debug)]
//pub struct Task {
//    /// The string to reverse
//    pub some_action: Option<String>,
//}

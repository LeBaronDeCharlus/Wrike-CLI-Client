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
    //Task(Task)
    Tasks(Tasks),
    Folders(Folders),
}

#[derive(Args, Debug)]
pub struct Tasks {
    pub tasks_arg: Option<String>,
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
